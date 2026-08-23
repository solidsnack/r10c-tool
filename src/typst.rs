use ciborium::{de::from_reader, ser::into_writer};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use wasm_minimal_protocol::*;

use crate::descriptors::Descriptor;
use crate::float64;

initiate_protocol!();

#[derive(Debug, Deserialize, Serialize)]
struct Holder(u16);

impl From<float64::Descriptor> for Holder {
    fn from(value: float64::Descriptor) -> Self {
        let bits: u16 = value.into();
        Self(bits)
    }
}

impl TryFrom<Holder> for float64::Descriptor {
    type Error = String;

    fn try_from(value: Holder) -> Result<Self, Self::Error> {
        let bits = value.0;

        bits.try_into()
            .map_err(|_| format!("Bad descriptor: {:08x}", bits))
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum Request {
    Near(f64),
    Resolve(Holder),
    Step(Holder, isize),
}

#[wasm_func]
pub fn request(data: &[u8]) -> Result<Vec<u8>, String> {
    use Request::*;

    let req: Request = from_cbor(data)?;

    let bytes = match req {
        Near(n) => {
            let d = float64::Descriptor::near(n);
            let h: Option<Holder> = d.map(|d| d.into());
            as_cbor(&h)
        }
        Resolve(h) => {
            let d: float64::Descriptor = h.try_into()?;
            let n = d.resolve();
            as_cbor(&n)
        }
        Step(h, i) => {
            let d: float64::Descriptor = h.try_into()?;
            let d2 = d.step(i);
            let h2: Option<Holder> = d2.map(|d| d.into());
            as_cbor(&h2)
        }
    }?;

    Ok(bytes)
}

fn as_cbor<T: ?Sized + Serialize>(value: &T) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    into_writer(&value, &mut out).map_err(|e| format!("CBOR: {e}"))?;
    Ok(out.to_vec())
}

fn from_cbor<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    from_reader(bytes).map_err(|e| format!("CBOR: {e}"))
}
