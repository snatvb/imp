use crate::prelude::*;

pub fn as_strings<'js>(
    ctx: &js::Ctx<'js>,
    args: js::prelude::Rest<js::Value<'js>>,
) -> js::Result<Vec<String>> {
    args.iter()
        .enumerate()
        .map(|(i, v)| {
            let arg = StringArg::coerce_js(ctx, v, format_args!("paths[{i}]"))?;
            Ok::<_, js::Error>(arg.as_str().to_string())
        })
        .collect::<js::Result<Vec<_>>>()
}
