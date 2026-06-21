use crate::js;
use crate::utils::{convert_to_string, extract_trace};

#[js::function]
pub fn assert<'js>(
    ctx: js::Ctx<'js>,
    condition: bool,
    js::prelude::Rest(args): js::prelude::Rest<js::Value<'js>>,
) -> js::Result<()> {
    if !condition {
        let msg = if args.is_empty() {
            "assertion failed".to_string()
        } else {
            convert_to_string(&ctx, args.as_slice(), 3, false)
        };
        let stack = extract_trace(&ctx);
        eprintln!("ASSERTION FAILED: {msg}\n{stack}");
        let err_ctor: js::Constructor = ctx.globals().get("Error")?;
        let err: js::Object = err_ctor.construct((msg,))?;
        return Err(ctx.throw(err.into_value()));
    }
    Ok(())
}
