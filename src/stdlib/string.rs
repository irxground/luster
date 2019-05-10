use gc_arena::MutationContext;
use gc_sequence as sequence;

use crate::{Callback, CallbackResult, Root, RuntimeError, String, Table, Value};

pub fn load_string<'gc>(mc: MutationContext<'gc, '_>, _: Root<'gc>, env: Table<'gc>) {
    let string = Table::new(mc);

    string
        .set(
            mc,
            "len",
            Callback::new_sequence(mc, |args| {
                Ok(sequence::from_fn_with(args, |mc, args| {
                    match args.get(0).cloned().unwrap_or(Value::Nil).to_string(mc) {
                        Some(s) => Ok(CallbackResult::Return(vec![Value::Integer(s.len())])),
                        None => Err(RuntimeError(Value::String(String::new_static(
                            b"Bad argument to len",
                        )))
                        .into()),
                    }
                }))
            }),
        )
        .unwrap();

    env.set(mc, "string", string).unwrap();
}
