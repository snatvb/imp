use std::process::Stdio;
use std::time::Instant;

use js_core::js;
use js_core::utils::{JsStringArg, StringArg};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

use crate::error::SubprocessError;
use crate::options::RunOptions;

#[js::function]
pub async fn run<'js>(
    ctx: js::Ctx<'js>,
    cmd: StringArg,
    args: js::Array<'js>,
    options: js::function::Opt<RunOptions>,
) -> js::Result<js::Object<'js>> {
    use js_core::error::{JsError, SystemError};

    let cmd_str = cmd.as_str().to_string();
    let opts = options.0.unwrap_or_default();

    let mut arg_strs = Vec::with_capacity(args.len());
    for v in args.iter::<js::Value<'js>>() {
        let val = v?;
        arg_strs.push(StringArg::coerce_string(&ctx, &val, "arg")?);
    }

    let timeout = opts.timeout;
    let has_input = opts.input.is_some();
    let input_bytes = opts.input.as_deref().unwrap_or("").as_bytes().to_vec();

    let mut command = Command::new(&cmd_str);
    for a in &arg_strs {
        command.arg(a);
    }
    if let Some(cwd) = &opts.cwd {
        command.current_dir(cwd);
    }
    if let Some(env) = &opts.env {
        command.env_clear();
        for (k, v) in env {
            command.env(k, v);
        }
    } else {
        command.env_clear();
    }
    if has_input {
        command.stdin(Stdio::piped());
    } else {
        command.stdin(Stdio::null());
    }
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    #[cfg(windows)]
    {
        command.creation_flags(0x08000000);
    }

    let start = Instant::now();
    let mut child = command
        .spawn()
        .map_err(|e| SubprocessError::from_io_spawn(e, &cmd_str).into_exception(&ctx))?;

    let mut stdin = child.stdin.take();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let stdin_task: Option<tokio::task::JoinHandle<()>> = if has_input {
        let bytes = input_bytes;
        Some(tokio::spawn(async move {
            if let Some(ref mut s) = stdin {
                let _ = s.write_all(&bytes).await;
                let _ = s.shutdown().await;
            }
        }))
    } else {
        None
    };

    let stdout_handle = stdout.ok_or_else(|| -> js::Error {
        SystemError {
            code: "UNKNOWN",
            syscall: "stdout",
            message: "failed to capture stdout".to_string(),
            path: None,
        }
        .into_exception(&ctx)
    })?;
    let stderr_handle = stderr.ok_or_else(|| -> js::Error {
        SystemError {
            code: "UNKNOWN",
            syscall: "stderr",
            message: "failed to capture stderr".to_string(),
            path: None,
        }
        .into_exception(&ctx)
    })?;

    let read_stdout_fut = read_capped(stdout_handle, opts.max_output());
    let read_stderr_fut = read_capped(stderr_handle, opts.max_output());

    let timeout_dur = timeout.map(std::time::Duration::from_millis);

    let (out_res, err_res) = if let Some(dur) = timeout_dur {
        match tokio::time::timeout(dur, async {
            tokio::join!(read_stdout_fut, read_stderr_fut)
        })
        .await
        {
            Ok(v) => v,
            Err(_) => {
                let _ = child.start_kill();
                let _ = child.wait().await;
                return Err(SubprocessError::Timeout(timeout.unwrap_or(0)).into_exception(&ctx));
            }
        }
    } else {
        tokio::join!(read_stdout_fut, read_stderr_fut)
    };

    let (out, out_capped) = out_res.map_err(|e| {
        SubprocessError::Io(SystemError::from_io(e, "read", Some(cmd_str.clone())))
            .into_exception(&ctx)
    })?;
    let (err, err_capped) = err_res.map_err(|e| {
        SubprocessError::Io(SystemError::from_io(e, "read", Some(cmd_str.clone())))
            .into_exception(&ctx)
    })?;

    if out_capped || err_capped {
        let _ = child.start_kill();
    }
    let status = child.wait().await.map_err(|e| {
        SubprocessError::Io(SystemError::from_io(e, "wait", Some(cmd_str.clone())))
            .into_exception(&ctx)
    })?;

    if let Some(t) = stdin_task {
        let _ = t.await;
    }

    let elapsed_ms = start.elapsed().as_millis() as i64;
    let code = status.code().unwrap_or(-1);
    let success = status.success();

    let result = js::Object::new(ctx.clone())?;
    result.set("code", code)?;
    result.set("stdout", String::from_utf8_lossy(&out).into_owned())?;
    result.set("stderr", String::from_utf8_lossy(&err).into_owned())?;
    result.set("success", success)?;
    result.set("durationMs", elapsed_ms)?;
    Ok(result)
}

async fn read_capped<R: AsyncReadExt + Unpin>(
    reader: R,
    cap: usize,
) -> std::io::Result<(Vec<u8>, bool)> {
    let mut limited = reader.take(cap as u64);
    let mut buf = Vec::with_capacity(cap.min(8192));
    limited.read_to_end(&mut buf).await?;
    let hit_cap = buf.len() >= cap;
    Ok((buf, hit_cap))
}
