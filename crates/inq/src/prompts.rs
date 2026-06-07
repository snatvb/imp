use inquire::{Select, Text};

use crate::prelude::*;

#[js::function]
pub async fn prompt<'js>(ctx: js::Ctx<'js>, text: js::Value<'js>) -> js::Result<String> {
    let text = StringArg::coerce_js(&ctx, &text, "text")?;

    tokio::task::spawn_blocking(move || Text::new(text.as_str()).prompt())
        .await
        .into_js(&ctx)? // tokio handle error
        .into_js(&ctx) // inquire error
}

#[js::function]
pub async fn select<'js>(
    ctx: js::Ctx<'js>,
    question: js::Value<'js>,
    variants: js::Array<'js>,
) -> js::Result<String> {
    let question = StringArg::coerce_js(&ctx, &question, "question")?;
    let iter = StringArg::coerce_array_iter(&ctx, &variants, "variants");
    let variants: Vec<String> = iter
        .map(|arg| arg.map(|a| a.as_str().to_string()))
        .collect::<js::Result<_>>()?;

    tokio::task::spawn_blocking(move || Select::new(question.as_str(), variants).prompt())
        .await
        .into_js(&ctx)? // tokio handle error
        .into_js(&ctx) // inquire error
}
