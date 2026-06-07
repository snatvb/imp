use inquire::Text;

use crate::prelude::*;

#[js::function]
pub async fn prompt<'js>(ctx: js::Ctx<'js>, text: js::Value<'js>) -> js::Result<String> {
    let text = StringArg::coerce_js(&ctx, &text, "text")?;

    tokio::task::spawn_blocking(move || Text::new(text.as_str()).prompt())
        .await
        .into_js(&ctx)? // tokio handle error
        .into_js(&ctx) // inquire error
}
