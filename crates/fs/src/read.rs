use js::IntoJs;
use js::function;
use js_core::RsString;
use js_core::js::Class;
use js_core::utils::StringArg;

use crate::prelude::*;

#[function]
pub async fn read_file<'js>(
    ctx: js::Ctx<'js>,
    path: StringArg,
    encoding: function::Opt<String>,
) -> js::Result<js::Value<'js>> {
    let result = crate::imp::read::read_file(ctx.clone(), path, encoding).await?;

    if let Ok(rs) = Class::<RsString>::from_value(&result) {
        let rs = rs.borrow();
        let s = rs.get_slice().to_string();
        Ok(s.into_js(&ctx)?)
    } else {
        Ok(result)
    }
}
