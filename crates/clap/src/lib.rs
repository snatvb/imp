pub mod arg_params;
pub mod error;
mod prelude;

use std::sync::Arc;

use clap::{Arg, Id};

use crate::{arg_params::ArgParams, prelude::*};

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
        let options = ArgParams::from_js(&ctx, options)?;
        let mut arg = Arg::new(options.name);
        if let Some(count) = options.count {
            arg = arg.num_args(1..count);
        }
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
