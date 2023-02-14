use super::*;
use mt_ser::{DeserializeError, SerializeError};
use std::io::{Read, Write};

#[mt_derive(to = "clt", custom)]
pub struct Inventory; // TODO

#[cfg(feature = "server")]
impl MtSerialize for Inventory {
    fn mt_serialize<C: MtCfg>(&self, writer: &mut impl Write) -> Result<(), SerializeError> {
        "EndInventory\n".mt_serialize::<()>(writer)
    }
}

fn read_line(reader: &mut impl Read) -> Result<String, DeserializeError> {
    let utf8 = mt_ser::mt_deserialize_seq::<(), u8>(reader)?
        .map_while(|x| match x {
            Ok(0x0A) => None,
            x => Some(x),
        })
        .try_collect::<Vec<_>>()?;

    String::from_utf8(utf8)
        .map_err(|e| DeserializeError::Other(format!("Invalid UTF-8: {e}").into()))
}

#[cfg(feature = "client")]
impl MtDeserialize for Inventory {
    fn mt_deserialize<C: MtCfg>(reader: &mut impl Read) -> Result<Self, DeserializeError> {
        loop {
            match read_line(reader)?.as_str() {
                "EndInventory" => return Ok(Self),
                _ => {}
            }
        }
    }
}
