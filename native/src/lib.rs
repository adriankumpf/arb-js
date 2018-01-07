extern crate abacom_relay_board;
#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::{JsArray, JsBoolean, JsNumber, JsUndefined};
use neon::js::error::{JsError, Kind};

fn activate(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let args = call.arguments;

    let relays_array = args.require(scope, 0)?.check::<JsArray>()?.to_vec(scope);

    let verify = args.require(scope, 1)?.check::<JsBoolean>()?.value();

    let port = match args.require(scope, 2)?.check::<JsNumber>() {
        Err(_) => None,
        Ok(p) => Some(p.value() as u8),
    };

    let mut relays = Vec::new();

    for r in relays_array?.iter() {
        let relay = r.check::<JsNumber>()?.value();
        relays.push(relay);
    }

    let relays = relays
        .iter()
        .filter(|&&r| r != 0.0)
        .map(|&r| r as u8)
        .fold(0, |acc, r| acc | 1 << (r - 1));

    if let Err(err) = abacom_relay_board::switch_relays(relays, verify, port) {
        return JsError::throw(Kind::Error, format!("{}", err).as_str());
    }

    Ok(JsUndefined::new())
}

register_module!(m, { m.export("activate", activate) });
