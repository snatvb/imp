use std::process::Stdio;
use std::time::Instant;

use js_core::js;
use js_core::utils::{JsStringArg, StringArg};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};

use crate::error::SubprocessError;
use crate::options::RunOptions;

#[inline]
fn io_err<'js>(
    ctx: &js::Ctx<'js>,
    e: std::io::Error,
    syscall: &'static str,
    cmd: &str,
) -> js::Error {
    use js_core::error::{JsError, SystemError};
    SubprocessError::Io(SystemError::from_io(e, syscall, Some(cmd.to_string()))).into_exception(ctx)
}

#[inline]
fn missing_pipe_err<'js>(ctx: &js::Ctx<'js>, kind: &'static str) -> js::Error {
    use js_core::error::{JsError, SystemError};
    SystemError {
        code: "UNKNOWN",
        syscall: kind,
        message: format!("failed to capture {kind}"),
        path: None,
    }
    .into_exception(ctx)
}

async fn force_kill(child: &mut Child) {
    if let Err(e) = child.start_kill() {
        eprintln!("subprocess: start_kill failed: {e}");
    }
    if let Err(e) = child.wait().await {
        eprintln!("subprocess: wait-after-kill failed: {e}");
    }
}

#[inline]
fn parse_args<'js>(ctx: &js::Ctx<'js>, args: js::Array<'js>) -> js::Result<Vec<String>> {
    StringArg::coerce_array_iter(ctx, &args, "arg")
        .map(|r| r.map(|s| s.as_str().to_string()))
        .collect()
}

fn build_command(cmd: &str, args: &[String], opts: &RunOptions) -> Command {
    let mut command = Command::new(cmd);
    for a in args {
        command.arg(a);
    }
    if let Some(cwd) = &opts.cwd {
        command.current_dir(cwd);
    }
    command.env_clear();
    if let Some(env) = &opts.env {
        for (k, v) in env {
            command.env(k, v);
        }
    }
    command.stdin(if opts.input.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    });
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    #[cfg(windows)]
    {
        command.creation_flags(0x08000000);
    }
    command
}

#[inline]
fn take_pipes<'js>(
    ctx: &js::Ctx<'js>,
    child: &mut Child,
) -> js::Result<(Option<ChildStdin>, ChildStdout, ChildStderr)> {
    let stdin = child.stdin.take();
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| missing_pipe_err(ctx, "stdout"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| missing_pipe_err(ctx, "stderr"))?;
    Ok((stdin, stdout, stderr))
}

#[inline]
fn spawn_stdin_writer(
    stdin: Option<ChildStdin>,
    input: Option<String>,
) -> Option<tokio::task::JoinHandle<()>> {
    let mut stdin = stdin?;
    let bytes = input?.into_bytes();
    Some(tokio::spawn(async move {
        if let Err(e) = stdin.write_all(&bytes).await {
            eprintln!("subprocess: stdin write failed: {e}");
        }
        if let Err(e) = stdin.shutdown().await {
            eprintln!("subprocess: stdin shutdown failed: {e}");
        }
    }))
}

async fn read_capped<R>(reader: R, cap: usize) -> std::io::Result<(Vec<u8>, bool)>
where
    R: AsyncReadExt + Unpin,
{
    let mut limited = reader.take(cap as u64);
    let mut buf = Vec::with_capacity(cap.min(8192));
    limited.read_to_end(&mut buf).await?;
    let hit_cap = buf.len() >= cap;
    Ok((buf, hit_cap))
}

async fn read_outputs<'js>(
    ctx: &js::Ctx<'js>,
    child: &mut Child,
    stdout: ChildStdout,
    stderr: ChildStderr,
    timeout: Option<u64>,
    cap: usize,
    cmd: &str,
) -> js::Result<(Vec<u8>, Vec<u8>)> {
    use js_core::error::JsError;

    let read_out = read_capped(stdout, cap);
    let read_err = read_capped(stderr, cap);

    let (out_res, err_res) = if let Some(ms) = timeout {
        match tokio::time::timeout(std::time::Duration::from_millis(ms), async {
            tokio::join!(read_out, read_err)
        })
        .await
        {
            Ok(v) => v,
            Err(_) => {
                force_kill(child).await;
                return Err(SubprocessError::Timeout(ms).into_exception(ctx));
            }
        }
    } else {
        tokio::join!(read_out, read_err)
    };

    let (out, out_capped) = out_res.map_err(|e| io_err(ctx, e, "read", cmd))?;
    let (err, err_capped) = err_res.map_err(|e| io_err(ctx, e, "read", cmd))?;
    if out_capped || err_capped {
        force_kill(child).await;
    }
    Ok((out, err))
}

fn build_result<'js>(
    ctx: &js::Ctx<'js>,
    status: std::process::ExitStatus,
    out: Vec<u8>,
    err: Vec<u8>,
    elapsed_ms: i64,
) -> js::Result<js::Object<'js>> {
    let result = js::Object::new(ctx.clone())?;
    result.set("code", status.code().unwrap_or(-1))?;
    result.set("success", status.success())?;
    result.set("stdout", String::from_utf8_lossy(&out).into_owned())?;
    result.set("stderr", String::from_utf8_lossy(&err).into_owned())?;
    result.set("durationMs", elapsed_ms)?;
    Ok(result)
}

#[js::function]
pub async fn run<'js>(
    ctx: js::Ctx<'js>,
    cmd: StringArg,
    args: js::Array<'js>,
    options: js::function::Opt<RunOptions>,
) -> js::Result<js::Object<'js>> {
    let cmd_str = cmd.as_str().to_string();
    let opts = options.0.unwrap_or_default();
    let arg_strs = parse_args(&ctx, args)?;

    let start = Instant::now();
    let mut command = build_command(&cmd_str, &arg_strs, &opts);
    let mut child = command
        .spawn()
        .map_err(|e| io_err(&ctx, e, "spawn", &cmd_str))?;

    let (stdin, stdout, stderr) = take_pipes(&ctx, &mut child)?;
    let stdin_task = spawn_stdin_writer(stdin, opts.input.clone());

    let (out, err) = read_outputs(
        &ctx,
        &mut child,
        stdout,
        stderr,
        opts.timeout,
        opts.max_output(),
        &cmd_str,
    )
    .await?;

    let status = child
        .wait()
        .await
        .map_err(|e| io_err(&ctx, e, "wait", &cmd_str))?;

    if let Some(t) = stdin_task
        && let Err(e) = t.await
    {
        eprintln!("subprocess: stdin task join failed: {e}");
    }

    build_result(&ctx, status, out, err, start.elapsed().as_millis() as i64)
}
