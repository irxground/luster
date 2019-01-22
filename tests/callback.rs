use luster::{
    compile, sequence_fn, Callback, Closure, ContinuationResult, Error, Lua, SequenceExt, Value,
};

#[test]
fn callback() -> Result<(), Box<Error>> {
    let mut lua = Lua::new();
    lua.sequence(|_| {
        Box::new(
            sequence_fn(|mc, lc| -> Result<(), Error> {
                let callback = Callback::new(mc, |args| {
                    let mut ret = args.to_vec();
                    ret.push(Value::Integer(42));
                    Ok(ContinuationResult::Finish(ret))
                });
                lc.globals.set(
                    mc,
                    lc.interned_strings.new_string(mc, b"callback"),
                    callback,
                )?;
                Ok(())
            })
            .and_then(|mc, lc, _| {
                Ok(Closure::new(
                    mc,
                    compile(
                        mc,
                        lc.interned_strings,
                        &br#"
                    local a, b, c = callback(1, 2)
                    return a == 1 and b == 2 and c == 42
                "#[..],
                    )?,
                    Some(lc.globals),
                )?)
            })
            .and_then(|mc, lc, closure| lc.main_thread.run_function(mc, closure, &[], 64))
            .map(|b| assert_eq!(b, vec![Value::Boolean(true)])),
        )
    })?;

    Ok(())
}