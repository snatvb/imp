use js_core::abort::AbortSignal;
use js_core::error::throw_abort_error;
use js_core::js;
use js_core::js::function::Opt;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::client::get_client;
use crate::file_fetch::file_fetch;
use crate::headers::Headers;
use crate::response::Response;
use crate::url::AnyURL;

#[js::function]
pub async fn fetch<'js>(
    ctx: js::Ctx<'js>,
    url: AnyURL,
    init: Opt<js::Object<'js>>,
) -> js::Result<Response> {
    let client = get_client();
    let mut method = reqwest::Method::GET;
    let mut req_headers = HeaderMap::new();
    let mut body: Option<String> = None;
    let mut signal: Option<AbortSignal> = None;

    if let Some(init) = init.0 {
        if let Ok(m) = init.get::<_, String>("method") {
            method = m.parse().unwrap_or(reqwest::Method::GET);
        }

        if let Ok(headers_obj) = init.get::<_, js::Object>("headers") {
            for prop in headers_obj.props::<String, String>() {
                let (key, value) = prop?;
                let name =
                    HeaderName::from_bytes(key.as_bytes()).map_err(|_| js::Error::Exception)?;
                let val = HeaderValue::from_str(&value).map_err(|_| js::Error::Exception)?;
                req_headers.insert(name, val);
            }
        }

        if let Ok(b) = init.get::<_, String>("body") {
            body = Some(b);
        }

        if let Ok(s) = init.get::<_, AbortSignal>("signal") {
            signal = Some(s);
        }
    }

    if let Some(ref sig) = signal
        && sig.is_aborted()
    {
        return Err(throw_abort_error(&ctx, "The operation was aborted"));
    }

    let url_str = url.as_str();

    if url_str.starts_with("file://") {
        return file_fetch(ctx, &url_str, signal).await;
    }

    let signal_token = signal.as_ref().map(|s| s.token().clone());

    let mut builder = client.request(method, &url_str).headers(req_headers);
    if let Some(b) = body {
        builder = builder.body(b);
    }

    let response = tokio::select! {
        resp = builder.send() => resp,
        _ = async {
            if let Some(t) = &signal_token {
                t.cancelled().await
            } else {
                std::future::pending::<()>().await
            }
        } => {
            return Err(throw_abort_error(&ctx, "The operation was aborted"));
        }
    };

    let response = response.map_err(|_| js::Error::Exception)?;

    let status = response.status().as_u16();
    let status_text = response
        .status()
        .canonical_reason()
        .unwrap_or("Unknown")
        .to_string();
    let resp_url = response.url().to_string();

    let mut resp_headers = Headers::new();
    for (key, value) in response.headers() {
        if let Ok(v) = value.to_str() {
            resp_headers.append(key.to_string(), v.to_string());
        }
    }

    let resp_body = response
        .bytes()
        .await
        .map_err(|_| js::Error::Exception)?
        .to_vec();

    Ok(Response::new(
        resp_body,
        status,
        status_text,
        resp_headers,
        resp_url,
    ))
}
