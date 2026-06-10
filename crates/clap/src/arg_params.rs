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
    pub long: Option<String>,
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

fn validate_non_neg(ctx: &js::Ctx<'_>, v: f64) -> js::Result<usize> {
    if !v.is_finite() || v.is_nan() || v < 0.0 {
        Err(Exception::throw_type(
            ctx,
            "num_args must be non-negative number",
        ))
    } else {
        Ok(v as usize)
    }
}

fn extract_num(ctx: &js::Ctx<'_>, arr: &js::Array<'_>, i: usize) -> js::Result<usize> {
    let v: f64 = arr
        .get::<js::Value>(i)?
        .as_number()
        .ok_or_else(|| Exception::throw_type(ctx, "num_args array elements must be numbers"))?;
    if !v.is_finite() || v.is_nan() || v < 0.0 {
        Err(Exception::throw_type(
            ctx,
            "num_args values must be non-negative",
        ))
    } else {
        Ok(v as usize)
    }
}

fn parse_num_args_array(ctx: &js::Ctx<'_>, arr: &js::Array<'_>) -> js::Result<ValueRange> {
    let len = arr.len();
    if len == 0 || len > 2 {
        return Err(Exception::throw_type(
            ctx,
            "num_args array must have 1 or 2 elements",
        ));
    }
    let first = extract_num(ctx, arr, 0)?;
    if len == 1 {
        return Ok(ValueRange::new(first..));
    }
    let second = extract_num(ctx, arr, 1)?;
    Ok(if first == 0 {
        ValueRange::new(..=second)
    } else {
        ValueRange::new(first..=second)
    })
}

fn parse_num_args<'js>(
    ctx: &js::Ctx<'js>,
    obj: &js::Object<'js>,
) -> js::Result<Option<ValueRange>> {
    let val = match obj.get::<_, Option<js::Value>>("num_args")? {
        Some(v) if !v.is_undefined() && !v.is_null() => v,
        _ => return Ok(None),
    };

    if let Some(n) = val.as_number() {
        return validate_non_neg(ctx, n).map(ValueRange::new).map(Some);
    }

    if let Some(arr) = val.as_array() {
        return parse_num_args_array(ctx, arr).map(Some);
    }

    Err(Exception::throw_type(
        ctx,
        "num_args must be a number or array",
    ))
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
        let obj = value.as_object().ok_or_else(|| {
            Exception::throw_type(
                ctx,
                &format!("Params must be an object but, got {}", value.type_name()),
            )
        })?;

        let name = StringArg::coerce_string(ctx, &obj.get("name")?, "name")?;
        let help = optional_string(ctx, obj, "help")?;
        let long = optional_string(ctx, obj, "long")?;

        let short = optional_string(ctx, obj, "short")?
            .map(|s| {
                let mut chars = s.chars();
                let c = chars.next().ok_or_else(|| {
                    Exception::throw_type(
                        ctx,
                        "Short must be exactly 1 character, got empty string",
                    )
                })?;
                if chars.next().is_some() {
                    return Err(Exception::throw_type(
                        ctx,
                        &format!("Short must be exactly 1 character, got \"{}\"", s),
                    ));
                }
                Ok(c)
            })
            .transpose()?;

        let exclusive: bool = obj.get::<_, Option<bool>>("exclusive")?.unwrap_or(false);
        let action_str = optional_string(ctx, obj, "action")?;
        let action = match action_str.as_deref() {
            Some(s) => Action::from_string(s)
                .ok_or_else(|| {
                    Exception::throw_type(ctx, &format!(
                        "Incorrect action value \"{}\", available: set, append, count, flag, set_false, help, help_short, help_long, version",
                        s,
                    ))
                })?,
            None => Action::default(),
        };

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
            long,
            help,
            exclusive,
            action,
        })
    }
}
