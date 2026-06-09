use js_core::utils::{JsStringArg, StringArg};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum Action {
    // TODO: Move here some non general fields
    #[default]
    Set,
    Append,
    Count,
    Flag,
    Help,
    Version,
}

impl Action {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "set" => Action::Set.into(),
            "append" => Action::Append.into(),
            "count" => Action::Count.into(),
            "flag" => Action::Flag.into(),
            "help" => Action::Help.into(),
            "version" => Action::Version.into(),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ArgParams {
    pub name: String,
    pub short: Option<String>,
    pub help: Option<String>,
    pub count: Option<i32>,
    pub choices: Option<Vec<String>>,
    pub action: Action,
    pub exclusive: bool,
}

fn optional_string<'js>(
    ctx: &js::Ctx<'js>,
    obj: &js::Object<'js>,
    name: &str,
) -> Result<Option<String>, js::Error> {
    obj.get::<_, Option<js::Value>>("short")?
        .filter(|val| !val.is_undefined() && !val.is_null())
        .map(|val| StringArg::coerce_string(ctx, &val, name))
        .transpose()
}

#[allow(clippy::option_as_ref_deref)]
impl ArgParams {
    pub fn from_js<'js>(ctx: &js::Ctx<'js>, value: js::Value<'js>) -> js::Result<Self> {
        let obj = value
            .as_object()
            .ok_or_else(|| {
                Error::TypeError(format!(
                    "Params must be an object but, got {}",
                    value.type_name()
                ))
            })
            .into_js(ctx)?;
        let name = StringArg::coerce_string(ctx, &obj.get("name")?, "name")?;

        let [short, help] = ["short", "help"].map(|n| optional_string(ctx, obj, n));

        let choices = obj
            .get::<_, Option<js::Array>>("count")?
            .as_ref()
            .map(|a| {
                StringArg::coerce_array_iter(ctx, a, "choices")
                    .map(|s| s.map(|x| x.to_string()))
                    .collect::<js::Result<_>>()
            })
            .transpose()?;
        let count = obj
            .get::<_, Option<js::Value>>("count")?
            .filter(|v| !v.is_undefined() && !v.is_null())
            .and_then(|v| v.as_number())
            .map(|num| -> Result<f64, js::Error> {
                if !num.is_finite() || num.is_nan() || num < 0.0 {
                    Err(Error::TypeError(
                        "Count must be positive integer, not NaN and Infinity".to_string(),
                    ))
                    .into_js(ctx)
                } else {
                    Ok(num)
                }
            })
            .transpose()?
            .map(|num| num as i32);

        let exclusive: bool = obj.get("exclusive")?;
        let action = optional_string(ctx, obj, "action")?;
        let action = action
            .as_ref()
            .map(String::as_str)
            .map(Action::from_string)
            .map(|s| {
                s.ok_or_else(|| {
                    Error::TypeError(format!(
                        "Incorrect action value \"{}\", available: set, append, count, flag, version",
                        action.unwrap_or_default(),
                    ))
                })
            }).transpose().into_js(ctx)?.unwrap_or_default();

        Ok(Self {
            name,
            count,
            short: short?,
            help: help?,
            choices,
            exclusive,
            action,
        })
    }
}
