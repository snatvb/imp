use chrono::{NaiveDate, Weekday};
use inquire::{DateSelect, Editor, MultiSelect, Password, Select, Text};
use js_core::utils::date::{naive_date_to_js_date, parse_date_field, weekday_from_u8};

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

#[inline(always)]
pub async fn ask_password<'js>(
    ctx: js::Ctx<'js>,
    question: js::Value<'js>,
    hidden: bool,
    with_confirm: bool,
) -> js::Result<String> {
    let text = StringArg::coerce_js(&ctx, &question, "text")?;
    let mode = if hidden {
        inquire::PasswordDisplayMode::Hidden
    } else {
        inquire::PasswordDisplayMode::Masked
    };

    spawn_blocking(&ctx, move || {
        let pass = Password::new(text.as_str()).with_display_mode(mode);
        let pass = if with_confirm {
            pass
        } else {
            pass.without_confirmation()
        };
        pass.prompt()
    })
    .await?
    .into_js(&ctx)
}

#[js::function]
pub async fn password<'js>(
    ctx: js::Ctx<'js>,
    question: js::Value<'js>,
    hidden: js::function::Opt<bool>,
) -> js::Result<String> {
    ask_password(ctx, question, hidden.unwrap_or(false), false).await
}

#[js::function]
pub async fn password_with_confirm<'js>(
    ctx: js::Ctx<'js>,
    question: js::Value<'js>,
    hidden: js::function::Opt<bool>,
) -> js::Result<String> {
    ask_password(ctx, question, hidden.unwrap_or(false), true).await
}

#[js::function]
pub async fn editor<'js>(ctx: js::Ctx<'js>, question: js::Value<'js>) -> js::Result<String> {
    let question = StringArg::coerce_js(&ctx, &question, "text")?;
    spawn_blocking(&ctx, move || Editor::new(question.as_str()).prompt())
        .await?
        .into_js(&ctx)
}

#[derive(Debug, Default)]
pub struct DateOptions {
    pub default: Option<NaiveDate>,
    pub min_date: Option<NaiveDate>,
    pub max_date: Option<NaiveDate>,
    pub week_start: Option<Weekday>,
    pub help_message: Option<String>,
}

impl DateOptions {
    pub fn from_js<'js>(_ctx: &js::Ctx<'js>, options: Option<js::Object<'js>>) -> js::Result<Self> {
        let Some(opts) = options else {
            return Ok(Self::default());
        };

        Ok(Self {
            default: parse_date_field(&opts, "default")?,
            min_date: parse_date_field(&opts, "minDate")?,
            max_date: parse_date_field(&opts, "maxDate")?,
            week_start: opts
                .get::<_, Option<u8>>("weekStart")?
                .and_then(weekday_from_u8),
            help_message: opts.get::<_, Option<String>>("helpMessage")?,
        })
    }
}

#[js::function]
pub async fn date_select<'js>(
    ctx: js::Ctx<'js>,
    question: js::Value<'js>,
    options: js::function::Opt<js::Object<'js>>,
) -> js::Result<js::Object<'js>> {
    let question = StringArg::coerce_js(&ctx, &question, "text")?;
    let opts = DateOptions::from_js(&ctx, options.into_inner())?;

    let result = spawn_blocking(&ctx, move || {
        let mut ds = DateSelect::new(question.as_str());

        if let Some(default) = opts.default {
            ds = ds.with_default(default);
        }
        if let Some(min) = opts.min_date {
            ds = ds.with_min_date(min);
        }
        if let Some(max) = opts.max_date {
            ds = ds.with_max_date(max);
        }
        if let Some(week_start) = opts.week_start {
            ds = ds.with_week_start(week_start);
        }
        if let Some(help) = &opts.help_message {
            ds = ds.with_help_message(help);
        }

        ds.prompt()
    })
    .await?
    .into_js(&ctx)?;

    naive_date_to_js_date(&ctx, &result)
}
