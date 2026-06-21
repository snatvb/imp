use crate::js;
use crate::utils::convert_to_string;

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
        eprintln!("ASSERTION FAILED: {}", msg);
        let err_ctor: js::Constructor = ctx.globals().get("Error")?;
        let err: js::Object = err_ctor.construct((msg,))?;
        return Err(ctx.throw(err.into_value()));
    }
    Ok(())
}
