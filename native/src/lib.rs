extern crate abacom_relay_board;
#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult, Throw};
use neon::js::{JsArray, JsBoolean, JsNumber, JsUndefined};
use neon::js::error::{JsError, Kind};

struct Args {
    relays: u8,
    port: Option<u8>,
    verify: bool,
}

fn parse_args(call: Call) -> Result<Args, Throw> {
    let scope = call.scope;
    let args = call.arguments;

    let mut relays = Vec::new();

    for r in args.require(scope, 0)?.check::<JsArray>()?.to_vec(scope)? {
        relays.push(r.check::<JsNumber>()?.value());
    }

    let relays = relays
        .iter()
        .map(|&r| r as u8)
        .filter(|&r| r != 0)
        .fold(0, |acc, r| acc | 1 << (r - 1));

    let verify = args.require(scope, 1)?.check::<JsBoolean>()?.value();

    let port = args.require(scope, 2)?
        .check::<JsNumber>()
        .map(|p| p.value() as u8)
        .ok();

    Ok(Args {
        relays,
        verify,
        port,
    })
}

fn activate(call: Call) -> JsResult<JsUndefined> {
    let args = parse_args(call)?;

    if let Err(err) = abacom_relay_board::switch_relays(args.relays, args.verify, args.port) {
        return JsError::throw(Kind::Error, format!("{}", err).as_str());
    }

    Ok(JsUndefined::new())
}

register_module!(m, { m.export("activate", activate) });
