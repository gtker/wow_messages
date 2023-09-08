use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/world_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/world_common.wowm#L3):
/// ```text
/// struct WorldState {
///     u32 state;
///     u32 value;
/// }
/// ```
pub struct WorldState {
    pub state: u32,
    pub value: u32,
}

impl WorldState {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // state: u32
        w.write_all(&self.state.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }
}

impl WorldState {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // state: u32
        let state = crate::util::read_u32_le(&mut r)?;

        // value: u32
        let value = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            state,
            value,
        })
    }

}

