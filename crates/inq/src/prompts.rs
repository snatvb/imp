use inquire::{MultiSelect, Select, Text};

use crate::prelude::*;

#[inline(always)]
async fn spawn_blocking<'js, R: Send + 'static>(
    ctx: &js::Ctx<'js>,
    f: impl FnOnce() -> R + Send + 'static,
) -> js::Result<R> {
    tokio::task::spawn_blocking(f).await.into_js(ctx)
}

#[js::function]
pub async fn prompt<'js>(ctx: js::Ctx<'js>, text: js::Value<'js>) -> js::Result<String> {
    let text = StringArg::coerce_js(&ctx, &text, "text")?;

    spawn_blocking(&ctx, move || Text::new(text.as_str()).prompt())
        .await?
        .into_js(&ctx)
}

fn to_string_select<'js>(
    ctx: &js::Ctx<'js>,
    question: js::Value<'js>,
    variants: js::Array<'js>,
) -> js::Result<(StringArg, Vec<String>)> {
    let question = StringArg::coerce_js(ctx, &question, "question")?;
    let iter = StringArg::coerce_array_iter(ctx, &variants, "variants");
    let variants: Vec<String> = iter
        .map(|arg| arg.map(|a| a.as_str().to_string()))
        .collect::<js::Result<_>>()?;
    Ok((question, variants))
}

#[js::function]
pub async fn select<'js>(
    ctx: js::Ctx<'js>,
    question: js::Value<'js>,
    variants: js::Array<'js>,
) -> js::Result<String> {
    let (question, variants) = to_string_select(&ctx, question, variants)?;

    spawn_blocking(&ctx, move || {
        Select::new(question.as_str(), variants).prompt()
    })
    .await?
    .into_js(&ctx)
}

#[js::function]
pub async fn multi_select<'js>(
    ctx: js::Ctx<'js>,
    question: js::Value<'js>,
    variants: js::Array<'js>,
) -> js::Result<Vec<String>> {
    let (question, variants) = to_string_select(&ctx, question, variants)?;

    spawn_blocking(&ctx, move || {
        MultiSelect::new(question.as_str(), variants).prompt()
    })
    .await?
    .into_js(&ctx)
}
