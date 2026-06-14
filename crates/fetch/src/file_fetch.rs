use js_core::abort::AbortSignal;
use js_core::error::JsError;
use js_core::js;

use crate::error::Error;
use crate::headers::Headers;
use crate::response::Response;

pub async fn file_fetch<'js>(
    ctx: js::Ctx<'js>,
    url: &str,
    signal: Option<AbortSignal>,
) -> js::Result<Response> {
    if let Some(ref sig) = signal
        && sig.is_aborted()
    {
        return Err(Error::Aborted("The operation was aborted".into()).into_exception(&ctx));
    }

    let path_str = file_path_from_url(url).map_err(|e| Error::Exception(e).into_exception(&ctx))?;

    let signal_token = signal.as_ref().map(|s| s.token().clone());

    let raw = tokio::select! {
        res = tokio::fs::read(&path_str) => res,
        _ = async {
            if let Some(t) = &signal_token {
                t.cancelled().await
            } else {
                std::future::pending::<()>().await
            }
        } => {
            return Err(
                Error::Aborted("The operation was aborted".into()).into_exception(&ctx),
            );
        }
    }
    .map_err(|e| {
        Error::System(js_core::error::SystemError::from_io(
            e,
            "fetch",
            Some(path_str.clone()),
        ))
        .into_exception(&ctx)
    })?;

    let content_type = guess_content_type(&path_str);
    let mut headers = Headers::new();
    headers.append("content-type".into(), content_type);

    Ok(Response::new(raw, 200, "OK".into(), headers, url.into()))
}

fn file_path_from_url(url: &str) -> Result<String, String> {
    let rest = &url["file://".len()..];

    let (authority, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => ("", rest),
    };

    let is_local = authority.is_empty() || authority.eq_ignore_ascii_case("localhost");

    if is_local {
        let path = path.strip_prefix('/').unwrap_or(path);
        let path = percent_decode(path);
        #[cfg(windows)]
        {
            Ok(os_path::OsPathBuf::new(path).to_string())
        }
        #[cfg(not(windows))]
        {
            Ok(format!("/{path}"))
        }
    } else {
        #[cfg(windows)]
        {
            let path = percent_decode(path);
            Ok(format!("\\\\{}{path}", authority.replace('/', "\\")))
        }
        #[cfg(not(windows))]
        {
            Err(format!(
                "file:// URLs with authority '{authority}' are not supported on non-Windows"
            ))
        }
    }
}

fn percent_decode(s: &str) -> String {
    let mut bytes = Vec::with_capacity(s.len());
    let mut iter = s.bytes();
    while let Some(b) = iter.next() {
        if b == b'%' {
            let hi = iter.next().unwrap_or(b'0');
            let lo = iter.next().unwrap_or(b'0');
            bytes.push((hex_digit(hi) << 4) | hex_digit(lo));
        } else {
            bytes.push(b);
        }
    }
    String::from_utf8(bytes).unwrap_or_default()
}

fn hex_digit(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}

fn guess_content_type(path: &str) -> String {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    match ext {
        "json" => "application/json",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "ts" => "application/typescript",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "wasm" => "application/wasm",
        "toml" => "application/toml",
        "yaml" | "yml" => "application/yaml",
        _ => "application/octet-stream",
    }
    .into()
}
