use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult
{
   Ok(cx.string("Привет из Rust!"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()>
{
   cx.export_function("hello", hello)?;
   Ok(())
}
