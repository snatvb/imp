pub mod arg_params;
pub mod error;
mod prelude;

use std::sync::OnceLock;

use clap::builder::ValueRange;
use clap::{Arg, ArgAction, Id};
use js_core::RsString;

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
    fn name(&mut self, name: String) {
        if let Some(cmd) = self.command.take() {
            self.command = Some(cmd.name(Id::from(name)));
        }
    }

    #[qjs()]
    fn version(&mut self, version: String) {
        if let Some(cmd) = self.command.take() {
            self.command = Some(cmd.version(Id::from(version)));
        }
    }

    #[qjs()]
    fn about(&mut self, about: String) {
        if let Some(cmd) = self.command.take() {
            self.command = Some(cmd.about(about));
        }
    }

    #[qjs()]
    fn arg<'js>(&mut self, ctx: js::Ctx<'js>, options: js::Value<'js>) -> js::Result<()> {
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
        Ok(())
    }

    #[qjs()]
    fn parse<'js>(&self, ctx: js::Ctx<'js>, args: Vec<String>) -> js::Result<js::Object<'js>> {
        let cmd = self
            .command
            .as_ref()
            .ok_or_else(|| Exception::throw_type(&ctx, "Parser not initialized"))?;

        let obj = js::Object::new(ctx.clone())?;

        match cmd.clone().try_get_matches_from(args) {
            Ok(matches) => {
                let rs_type = js::Class::instance(ctx.clone(), RsString::owned("result".to_string()))?;
                obj.set("type", rs_type)?;
                for arg in cmd.get_arguments() {
                    let name = arg.get_id().as_str();
                    match arg.get_action() {
                        ArgAction::Set => {
                            let num_args = arg.get_num_args().unwrap_or(ValueRange::new(1));
                            if num_args.max_values() == 1 {
                                if let Some(val) = matches.get_one::<String>(name) {
                                    let rs_str =
                                        js::Class::instance(ctx.clone(), RsString::owned(val.clone()))?;
                                    obj.set(name, rs_str)?;
                                }
                            } else {
                                let vals: Vec<js::Class<'js, RsString>> = matches
                                    .get_many::<String>(name)
                                    .unwrap_or_default()
                                    .map(|s| js::Class::instance(ctx.clone(), RsString::owned(s.clone())))
                                    .collect::<js::Result<_>>()?;
                                if !vals.is_empty() {
                                    obj.set(name, vals)?;
                                }
                            }
                        }
                        ArgAction::Append => {
                            let vals: Vec<js::Class<'js, RsString>> = matches
                                .get_many::<String>(name)
                                .unwrap_or_default()
                                .map(|s| js::Class::instance(ctx.clone(), RsString::owned(s.clone())))
                                .collect::<js::Result<_>>()?;
                            if !vals.is_empty() {
                                obj.set(name, vals)?;
                            }
                        }
                        ArgAction::Count => {
                            obj.set(name, matches.get_count(name))?;
                        }
                        ArgAction::SetTrue | ArgAction::SetFalse => {
                            obj.set(name, matches.get_flag(name))?;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                let type_str = match e.kind() {
                    clap::error::ErrorKind::DisplayHelp
                    | clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => "help",
                    clap::error::ErrorKind::DisplayVersion => "version",
                    _ => return Err(Exception::throw_type(&ctx, &e.to_string())),
                };
                let rs_type = js::Class::instance(ctx.clone(), RsString::owned(type_str.to_string()))?;
                obj.set("type", rs_type)?;
                let message = RsString::owned(e.render().to_string());
                let rs_message = js::Class::instance(ctx.clone(), message)?;
                obj.set("message", rs_message)?;
            }
        }

        Ok(obj)
    }
}

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
