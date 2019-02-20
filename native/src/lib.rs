#[macro_use]
extern crate neon;
use arb;

use neon::prelude::*;
use neon::result::Throw;

struct Args {
    relays: u8,
    port: Option<u8>,
    verify: bool,
}

fn parse_args(cx: &mut FunctionContext<'_>) -> Result<Args, Throw> {
    let mut relays = Vec::new();

    for r in cx.argument::<JsArray>(0)?.to_vec(cx)? {
        relays.push(r.downcast::<JsNumber>().or_throw(cx)?.value());
    }

    let relays = relays
        .iter()
        .map(|&r| r as u8)
        .filter(|&r| r != 0)
        .fold(0, |acc, r| acc | 1 << (r - 1));

    let verify = cx.argument::<JsBoolean>(1)?.value();
    let port = cx.argument::<JsNumber>(2).map(|p| p.value() as u8).ok();

    Ok(Args {
        relays,
        verify,
        port,
    })
}

fn activate(mut cx: FunctionContext<'_>) -> JsResult<'_, JsUndefined> {
    let args = parse_args(&mut cx)?;

    if let Err(err) = arb::set_status(args.relays, args.verify, args.port) {
        return cx.throw_error(format!("{}", err).as_str());
    }

    Ok(JsUndefined::new())
}

register_module!(mut cx, { cx.export_function("activate", activate) });
