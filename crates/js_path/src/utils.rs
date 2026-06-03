use crate::prelude::*;

pub fn as_strings<'js>(
    ctx: &js::Ctx<'js>,
    args: js::prelude::Rest<js::Value<'js>>,
) -> js::Result<Vec<String>> {
    args.iter()
        .enumerate()
        .map(|(i, v)| String::coerce_js(ctx, v, format_args!("paths[{i}]")))
        .collect::<js::Result<Vec<_>>>()
}
