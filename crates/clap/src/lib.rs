pub mod arg_params;
pub mod error;
mod prelude;

use std::sync::OnceLock;

use clap::builder::ValueRange;
use clap::{Arg, ArgAction, Id};
use js::function::This;
use js_core::RsString;
use js_core::utils::{JsStringArg, StringArg};

use crate::{
    arg_params::{Action, ArgParams},
    prelude::*,
};

static ARGS: OnceLock<Vec<RsString>> = OnceLock::new();

fn take_args<'js>(ctx: &js::Ctx<'js>) -> js::Result<js::Array<'js>> {
    let args = js::Array::new(ctx.clone())?;
    for (i, arg) in ARGS
        .get()
        .ok_or_else(|| Exception::throw_type(ctx, "imp_clap not initialized"))?
        .iter()
        .enumerate()
    {
        args.set(i, arg.clone())?;
    }

    Ok(args)
}

js_core::impl_module!(ClapModule,
    declare: |decl, declare_all| {
        decl.declare("Parser")?;
        decl.declare("default")?;
        decl.declare("args")?;
        Ok(())
    },
    evaluate: |ctx, exports, export_all| {
        js::Class::<Parser>::define(&ctx.globals())?;
        let ctor = js::Class::<Parser>::create_constructor(ctx)?
            .ok_or_else(|| Exception::throw_type(ctx, "Failed to create Parser constructor"))?;
        exports.export("Parser", ctor.clone())?;
        let args = take_args(ctx)?;
        exports.export("args", args.clone())?;
        let ns = export_all(ctx, exports)?;
        ns.set("Parser", ctor)?;
        ns.set("args", args)?;
        exports.export("default", ns)?;
        Ok(())
    },
);

#[derive(Debug, js::class::Trace, js::JsLifetime)]
#[js::class]
pub struct Parser {
    #[qjs(skip_trace)]
    command: Option<clap::Command>,
}

#[js::methods]
impl Parser {
    #[qjs(constructor)]
    fn new() -> Self {
        Parser {
            command: Some(clap::Command::new("app")),
        }
    }

    #[qjs()]
    fn name<'js>(
        &mut self,
        this: This<js::Class<'js, Parser>>,
        name: StringArg,
    ) -> js::Result<js::Class<'js, Parser>> {
        if let Some(cmd) = self.command.take() {
            self.command = Some(cmd.name(Id::from(name.to_string())));
        }
        Ok(this.0.clone())
    }

    #[qjs()]
    fn version<'js>(
        &mut self,
        this: This<js::Class<'js, Parser>>,
        version: StringArg,
    ) -> js::Result<js::Class<'js, Parser>> {
        if let Some(cmd) = self.command.take() {
            self.command = Some(cmd.version(Id::from(version.to_string())));
        }
        Ok(this.0.clone())
    }

    #[qjs()]
    fn about<'js>(
        &mut self,
        this: This<js::Class<'js, Parser>>,
        about: StringArg,
    ) -> js::Result<js::Class<'js, Parser>> {
        if let Some(cmd) = self.command.take() {
            self.command = Some(cmd.about(about.to_string()));
        }
        Ok(this.0.clone())
    }

    #[qjs()]
    fn arg<'js>(
        &mut self,
        this: This<js::Class<'js, Parser>>,
        ctx: js::Ctx<'js>,
        options: js::Value<'js>,
    ) -> js::Result<js::Class<'js, Parser>> {
        let params = ArgParams::from_js(&ctx, options)?;
        let mut arg = Arg::new(&params.name);

        if let Some(short) = params.short {
            arg = arg.short(short);
        }
        if let Some(long) = &params.long {
            arg = arg.long(long);
        }
        if let Some(help) = &params.help {
            arg = arg.help(help);
        }
        if params.exclusive {
            arg = arg.exclusive(true);
        }
        if params.required {
            arg = arg.required(true);
        }

        arg = match params.action {
            Action::Set { choices, num_args } => {
                let mut a = arg.action(clap::ArgAction::Set);
                if let Some(choices) = &choices {
                    a = a.value_parser(choices.to_vec());
                }
                if let Some(num_args) = num_args {
                    a = a.num_args(num_args);
                }
                a
            }
            Action::Append { choices, num_args } => {
                let mut a = arg.action(clap::ArgAction::Append);
                if let Some(choices) = &choices {
                    a = a.value_parser(choices.to_vec());
                }
                if let Some(num_args) = num_args {
                    a = a.num_args(num_args);
                }
                a
            }
            Action::Count => arg.action(clap::ArgAction::Count),
            Action::Flag => arg.action(clap::ArgAction::SetTrue),
            Action::SetFalse => arg.action(clap::ArgAction::SetFalse),
            Action::Help => arg.action(clap::ArgAction::Help),
            Action::HelpShort => arg.action(clap::ArgAction::HelpShort),
            Action::HelpLong => arg.action(clap::ArgAction::HelpLong),
            Action::Version => arg.action(clap::ArgAction::Version),
        };

        if let Some(cmd) = self.command.take() {
            self.command = Some(cmd.arg(arg));
        }
        Ok(this.0.clone())
    }

    #[qjs()]
    fn parse<'js>(&self, ctx: js::Ctx<'js>, args: js::Array<'js>) -> js::Result<js::Object<'js>> {
        let cmd = self
            .command
            .as_ref()
            .ok_or_else(|| Exception::throw_type(&ctx, "Parser not initialized"))?;

        let args_vec: Vec<String> = StringArg::coerce_array_iter(&ctx, &args, "args")
            .map(|r| r.map(|s| s.as_str().to_string()))
            .collect::<js::Result<_>>()?;

        let obj = js::Object::new(ctx.clone())?;

        let mut full_args = vec![self.command.as_ref().unwrap().get_name().to_string()];
        full_args.extend(args_vec);

        match cmd.clone().try_get_matches_from(full_args) {
            Ok(matches) => build_result(&ctx, &obj, cmd, &matches)?,
            Err(e) => build_error(&ctx, &obj, &e)?,
        }

        Ok(obj)
    }
}

#[inline(always)]
fn strings_to_js_vec<'js>(
    ctx: &js::Ctx<'js>,
    strings: impl Iterator<Item = impl AsRef<str>>,
) -> js::Result<Vec<js::String<'js>>> {
    strings
        .map(|s| js::String::from_str(ctx.clone(), s.as_ref()))
        .collect()
}

#[inline(always)]
fn handle_set_action<'js>(
    ctx: &js::Ctx<'js>,
    obj: &js::Object<'js>,
    matches: &clap::ArgMatches,
    arg: &clap::Arg,
) -> js::Result<()> {
    let name = arg.get_id().as_str();
    let num_args = arg.get_num_args().unwrap_or(ValueRange::new(1));

    if num_args.max_values() == 1 {
        if let Some(val) = matches.get_one::<String>(name) {
            obj.set(name, val.as_str())?;
        }
    } else {
        let vals = strings_to_js_vec(ctx, matches.get_many::<String>(name).unwrap_or_default())?;
        if !vals.is_empty() {
            obj.set(name, vals)?;
        }
    }
    Ok(())
}

#[inline(always)]
fn handle_append_action<'js>(
    ctx: &js::Ctx<'js>,
    obj: &js::Object<'js>,
    matches: &clap::ArgMatches,
    arg: &clap::Arg,
) -> js::Result<()> {
    let name = arg.get_id().as_str();
    let vals = strings_to_js_vec(ctx, matches.get_many::<String>(name).unwrap_or_default())?;
    if !vals.is_empty() {
        obj.set(name, vals)?;
    }
    Ok(())
}

fn build_result<'js>(
    ctx: &js::Ctx<'js>,
    obj: &js::Object<'js>,
    cmd: &clap::Command,
    matches: &clap::ArgMatches,
) -> js::Result<()> {
    obj.set("type", "ok")?;

    for arg in cmd.get_arguments() {
        match arg.get_action() {
            ArgAction::Set => handle_set_action(ctx, obj, matches, arg)?,
            ArgAction::Append => handle_append_action(ctx, obj, matches, arg)?,
            ArgAction::Count => {
                obj.set(
                    arg.get_id().as_str(),
                    matches.get_count(arg.get_id().as_str()),
                )?;
            }
            ArgAction::SetTrue | ArgAction::SetFalse => {
                obj.set(
                    arg.get_id().as_str(),
                    matches.get_flag(arg.get_id().as_str()),
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}

#[inline(always)]
fn build_error<'js>(_ctx: &js::Ctx<'js>, obj: &js::Object<'js>, e: &clap::Error) -> js::Result<()> {
    let type_str = match e.kind() {
        clap::error::ErrorKind::DisplayHelp
        | clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => "help",
        clap::error::ErrorKind::DisplayVersion => "version",
        _ => "error",
    };
    obj.set("type", type_str)?;
    obj.set("message", e.render().to_string().as_str())?;
    Ok(())
}

#[inline(always)]
pub fn init<'js>(_ctx: &js::Ctx<'js>, args: &[impl std::borrow::Borrow<str>]) -> js::Result<()> {
    let args = args
        .iter()
        .map(|s| s.borrow())
        .map(|s| s.to_owned())
        .map(RsString::owned)
        .collect::<Vec<_>>();
    ARGS.set(args).unwrap();

    Ok(())
}
