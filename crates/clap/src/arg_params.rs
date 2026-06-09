use clap::builder::ValueRange;
use js_core::utils::{JsStringArg, StringArg};

use crate::prelude::*;

#[derive(Debug)]
pub enum Action {
    Set {
        choices: Option<Vec<String>>,
        num_args: Option<ValueRange>,
    },
    Append {
        choices: Option<Vec<String>>,
        num_args: Option<ValueRange>,
    },
    Count,
    Flag,
    SetFalse,
    Help,
    HelpShort,
    HelpLong,
    Version,
}

impl Default for Action {
    fn default() -> Self {
        Self::Set {
            choices: None,
            num_args: None,
        }
    }
}

impl Action {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "set" => Action::Set {
                choices: None,
                num_args: None,
            }
            .into(),
            "append" => Action::Append {
                choices: None,
                num_args: None,
            }
            .into(),
            "count" => Action::Count.into(),
            "flag" => Action::Flag.into(),
            "set_false" => Action::SetFalse.into(),
            "help" => Action::Help.into(),
            "help_short" => Action::HelpShort.into(),
            "help_long" => Action::HelpLong.into(),
            "version" => Action::Version.into(),
            _ => None,
        }
    }
}

impl From<Action> for clap::ArgAction {
    fn from(action: Action) -> Self {
        match action {
            Action::Set { .. } => clap::ArgAction::Set,
            Action::Append { .. } => clap::ArgAction::Append,
            Action::Count => clap::ArgAction::Count,
            Action::Flag => clap::ArgAction::SetTrue,
            Action::SetFalse => clap::ArgAction::SetFalse,
            Action::Help => clap::ArgAction::Help,
            Action::HelpShort => clap::ArgAction::HelpShort,
            Action::HelpLong => clap::ArgAction::HelpLong,
            Action::Version => clap::ArgAction::Version,
        }
    }
}

#[derive(Debug)]
pub struct ArgParams {
    pub name: String,
    pub short: Option<char>,
    pub help: Option<String>,
    pub exclusive: bool,
    pub action: Action,
}

fn optional_string<'js>(
    ctx: &js::Ctx<'js>,
    obj: &js::Object<'js>,
    name: &str,
) -> Result<Option<String>, js::Error> {
    obj.get::<_, Option<js::Value>>(name)?
        .filter(|val| !val.is_undefined() && !val.is_null())
        .map(|val| StringArg::coerce_string(ctx, &val, name))
        .transpose()
}

fn parse_num_args<'js>(
    ctx: &js::Ctx<'js>,
    obj: &js::Object<'js>,
) -> js::Result<Option<ValueRange>> {
    let val = match obj.get::<_, Option<js::Value>>("num_args")? {
        Some(v) if !v.is_undefined() && !v.is_null() => v,
        _ => return Ok(None),
    };

    if let Some(num) = val.as_number() {
        if !num.is_finite() || num.is_nan() || num < 0.0 {
            return Err(
                Error::TypeError("num_args must be non-negative number".to_string())
                    .into_exception(ctx),
            );
        }
        return Ok(Some(ValueRange::new(num as usize)));
    }

    if let Some(arr) = val.as_array() {
        let len = arr.len();
        if len == 0 || len > 2 {
            return Err(
                Error::TypeError("num_args array must have 1 or 2 elements".to_string())
                    .into_exception(ctx),
            );
        }

        let first: f64 = arr
            .get::<js::Value>(0)?
            .as_number()
            .ok_or_else(|| Error::TypeError("num_args array elements must be numbers".to_string()))
            .into_js(ctx)?;

        if !first.is_finite() || first.is_nan() || first < 0.0 {
            return Err(
                Error::TypeError("num_args values must be non-negative".to_string())
                    .into_exception(ctx),
            );
        }
        let first = first as usize;

        if len == 1 {
            return Ok(Some(ValueRange::new(first..)));
        }

        let second: f64 = arr
            .get::<js::Value>(1)?
            .as_number()
            .ok_or_else(|| Error::TypeError("num_args array elements must be numbers".to_string()))
            .into_js(ctx)?;

        if !second.is_finite() || second.is_nan() || second < 0.0 {
            return Err(
                Error::TypeError("num_args values must be non-negative".to_string())
                    .into_exception(ctx),
            );
        }
        let second = second as usize;

        if first == 0 {
            return Ok(Some(ValueRange::new(..=second)));
        }
        return Ok(Some(ValueRange::new(first..=second)));
    }

    Err(Error::TypeError("num_args must be a number or array".to_string()).into_exception(ctx))
}

fn parse_choices<'js>(
    ctx: &js::Ctx<'js>,
    obj: &js::Object<'js>,
) -> js::Result<Option<Vec<String>>> {
    match obj.get::<_, Option<js::Array>>("choices")? {
        Some(arr) => {
            let choices: Vec<String> = StringArg::coerce_array_iter(ctx, &arr, "choices")
                .map(|s| s.map(|x| x.to_string()))
                .collect::<js::Result<_>>()?;
            Ok(Some(choices))
        }
        None => Ok(None),
    }
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

        let short = short?
            .map(|s| {
                let mut chars = s.chars();
                let c = chars.next().ok_or_else(|| {
                    Error::TypeError(
                        "Short must be exactly 1 character, got empty string".to_string(),
                    )
                })?;
                if chars.next().is_some() {
                    return Err(Error::TypeError(format!(
                        "Short must be exactly 1 character, got \"{}\"",
                        s
                    )));
                }
                Ok(c)
            })
            .transpose()
            .into_js(ctx)?;

        let exclusive: bool = obj.get("exclusive")?;
        let action_str = optional_string(ctx, obj, "action")?;
        let action = action_str
            .as_ref()
            .map(String::as_str)
            .map(Action::from_string)
            .map(|s| {
                s.ok_or_else(|| {
                    Error::TypeError(format!(
                        "Incorrect action value \"{}\", available: set, append, count, flag, set_false, help, help_short, help_long, version",
                        action_str.unwrap_or_default(),
                    ))
                })
            })
            .transpose()
            .into_js(ctx)?
            .unwrap_or_default();

        let action = match action {
            Action::Set { .. } => Action::Set {
                choices: parse_choices(ctx, obj)?,
                num_args: parse_num_args(ctx, obj)?,
            },
            Action::Append { .. } => Action::Append {
                choices: parse_choices(ctx, obj)?,
                num_args: parse_num_args(ctx, obj)?,
            },
            other => other,
        };

        Ok(Self {
            name,
            short,
            help: help?,
            exclusive,
            action,
        })
    }
}
