use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn semver_valid(mut cx: FunctionContext) -> JsResult<JsValue> {
    let require: Handle<JsFunction> = cx.argument(0)?;
    let null = cx.null();
    let args = vec![cx.string("semver")];
    let semver: Handle<JsObject> = require
        .call(&mut cx, null, args)?
        .downcast_or_throw(&mut cx)?;
    let valid: Handle<JsFunction> = semver.get(&mut cx, "valid")?.downcast_or_throw(&mut cx)?;

    let version = cx.argument_opt(1);
    valid.call(&mut cx, semver, version)
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("semver_valid", semver_valid)?;
    Ok(())
});
