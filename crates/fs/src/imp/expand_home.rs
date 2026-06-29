use std::path::PathBuf;

use js_core::utils::StringArg;

use crate::prelude::js;

#[js::function]
pub fn expand_home<'js>(ctx: js::Ctx<'js>, path: StringArg) -> js::Result<js::String<'js>> {
    let expanded = expand_home_impl(path.as_str());
    let s = expanded.to_string_lossy().into_owned();
    js::String::from_str(ctx.clone(), &s)
}

fn expand_home_impl(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix('~')
        && let Some(home) = dirs::home_dir()
    {
        let home = PathBuf::from(home.to_string_lossy().trim_end_matches('/'));

        if rest.is_empty() {
            return home;
        }
        if rest.starts_with('/') {
            let trimmed = rest.trim_start_matches('/');
            if trimmed.is_empty() {
                return home;
            }
            return home.join(trimmed);
        }
    }
    PathBuf::from(path)
}
