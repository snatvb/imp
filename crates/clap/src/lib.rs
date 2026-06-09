pub mod arg_params;
pub mod error;
mod prelude;

use std::sync::Arc;

use clap::{Arg, Id};

use crate::{
    arg_params::{Action, ArgParams},
    prelude::*,
};

js_core::impl_module!(ClapModule,
    evaluate: |ctx, exports, export_all| {
        init(ctx)?;
        let ns = export_all(ctx, exports)?;
        exports.export("default", ns)?;
        Ok(())
    },
);

pub fn init<'js>(ctx: &js::Ctx<'js>) -> js::Result<()> {
    js::Class::<Parser>::define(&ctx.globals())?;
    Ok(())
}

#[derive(Debug, js::class::Trace, js::JsLifetime)]
#[js::class]
pub struct Parser {
    #[qjs(skip_trace)]
    command: Option<clap::Command>,
}

fn update(
    this: js::class::Class<'_, Parser>,
    f: impl FnOnce(clap::Command) -> clap::Command,
) -> js::class::Class<'_, Parser> {
    let mut cell = this.borrow_mut();
    let cmd = cell.command.take();
    cell.command = cmd.map(f);
    drop(cell);
    this
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
    fn name(this: js::class::Class<'_, Parser>, name: String) -> js::class::Class<'_, Parser> {
        update(this, |c| c.name(Id::from(name)))
    }

    #[qjs()]
    fn version(
        this: js::class::Class<'_, Parser>,
        version: String,
    ) -> js::class::Class<'_, Parser> {
        update(this, |c| c.version(Id::from(version)))
    }

    #[qjs()]
    fn about(this: js::class::Class<'_, Parser>, about: String) -> js::class::Class<'_, Parser> {
        update(this, |c| c.about(about))
    }

    #[qjs()]
    fn arg<'js>(
        ctx: js::Ctx<'js>,
        this: js::class::Class<'js, Parser>,
        options: js::Value<'js>,
    ) -> js::Result<js::class::Class<'js, Parser>> {
        let params = ArgParams::from_js(&ctx, options)?;
        let mut arg = Arg::new(&params.name);

        if let Some(short) = params.short {
            arg = arg.short(short);
        }
        if let Some(help) = &params.help {
            let s: &'static str = Box::leak(help.clone().into_boxed_str());
            arg = arg.help(s);
        }
        if params.exclusive {
            arg = arg.exclusive(true);
        }

        arg = match params.action {
            Action::Set { choices, num_args } => {
                let mut a = arg.action(clap::ArgAction::Set);
                if let Some(choices) = &choices {
                    let strs: Vec<&'static str> = choices
                        .iter()
                        .map(|s| Box::leak(s.clone().into_boxed_str()) as &'static str)
                        .collect();
                    a = a.value_parser(strs);
                }
                if let Some(num_args) = num_args {
                    a = a.num_args(num_args);
                }
                a
            }
            Action::Append { choices, num_args } => {
                let mut a = arg.action(clap::ArgAction::Append);
                if let Some(choices) = &choices {
                    let strs: Vec<&'static str> = choices
                        .iter()
                        .map(|s| Box::leak(s.clone().into_boxed_str()) as &'static str)
                        .collect();
                    a = a.value_parser(strs);
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

        Ok(update(this, |c| c.arg(arg)))
    }
}

// TODO: Move to js_core
#[derive(Debug, js::JsLifetime)]
struct SavedArgs(Arc<RsString>);

// pub fn set_script_args<'js>(
//     ctx: &js::Ctx<'js>,
//     args: &[impl std::borrow::Borrow<str>],
// ) -> js::Result<()> {
//     ctx.store_userdata(SavedArgs(RsString::owned(args.join(" ")).into()))?;
//     Ok(())
// }
//
// #[js::function]
// fn parse<'js>(ctx: js::Ctx<'js>, args: js::function::Opt<js::Value<'js>>) -> js::Result<()> {
//     let args = args
//         .as_ref()
//         .as_ref()
//         .map(|val| StringArg::coerce_js(&ctx, val, "args"))
//         .unwrap_or_else(|| {
//             ctx.userdata::<SavedArgs>()
//                 .map(|saved| StringArg::RsString(saved.0.clone()))
//                 .ok_or_else(|| js::Error::new_from_js("userdata", "SavedArgs"))
//         })?;
//     todo!()
// }
