use rquickjs::function;

use crate::macros::make_path_wrappers;
use crate::prelude::*;

make_path_wrappers!(os_path::Posix, "/", ":");
