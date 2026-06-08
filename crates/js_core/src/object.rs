use rquickjs::function::{Args, IntoArgs};
use rquickjs::{FromJs, Function, Object, Result};

pub trait ObjectMethodExt<'js> {
    fn call_method<A, R>(&self, name: &str, args: A) -> Result<R>
    where
        A: IntoArgs<'js>,
        R: FromJs<'js>;
}

impl<'js> ObjectMethodExt<'js> for Object<'js> {
    fn call_method<A, R>(&self, name: &str, args: A) -> Result<R>
    where
        A: IntoArgs<'js>,
        R: FromJs<'js>,
    {
        let func: Function = self.get(name)?;

        let mut qjs_args = Args::new(func.ctx().clone(), 0);
        args.into_args(&mut qjs_args)?;
        qjs_args.this(self.clone())?;

        func.call_arg::<R>(qjs_args)
    }
}
