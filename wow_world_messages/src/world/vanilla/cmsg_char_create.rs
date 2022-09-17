use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Class;
use crate::world::vanilla::Gender;
use crate::world::vanilla::Race;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Sent after the client presses 'Create Character'. The client will then wait for [`SMSG_CHAR_CREATE`](crate::world::vanilla::SMSG_CHAR_CREATE).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm#L2):
/// ```text
/// cmsg CMSG_CHAR_CREATE = 0x0036 {
///     CString name;
///     Race race;
///     Class class;
///     Gender gender;
///     u8 skin_color;
///     u8 face;
///     u8 hair_style;
///     u8 hair_color;
///     u8 facial_hair;
///     u8 outfit_id = 0;
/// }
/// ```
pub struct CMSG_CHAR_CREATE {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub skin_color: u8,
    pub face: u8,
    pub hair_style: u8,
    pub hair_color: u8,
    pub facial_hair: u8,
}

impl CMSG_CHAR_CREATE {
    /// The field `outfit_id` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const OUTFIT_ID_VALUE: u8 = 0x00;

}

impl crate::Message for CMSG_CHAR_CREATE {
    const OPCODE: u32 = 0x0036;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        // Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0u8), "String name must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&(self.race.as_int() as u8).to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u8).to_le_bytes())?;

        // skin_color: u8
        w.write_all(&self.skin_color.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hair_style: u8
        w.write_all(&self.hair_style.to_le_bytes())?;

        // hair_color: u8
        w.write_all(&self.hair_color.to_le_bytes())?;

        // facial_hair: u8
        w.write_all(&self.facial_hair.to_le_bytes())?;

        // outfit_id: u8
        w.write_all(&Self::OUTFIT_ID_VALUE.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // race: Race
        let race: Race = crate::util::read_u8_le(r)?.try_into()?;

        // class: Class
        let class: Class = crate::util::read_u8_le(r)?.try_into()?;

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(r)?.try_into()?;

        // skin_color: u8
        let skin_color = crate::util::read_u8_le(r)?;

        // face: u8
        let face = crate::util::read_u8_le(r)?;

        // hair_style: u8
        let hair_style = crate::util::read_u8_le(r)?;

        // hair_color: u8
        let hair_color = crate::util::read_u8_le(r)?;

        // facial_hair: u8
        let facial_hair = crate::util::read_u8_le(r)?;

        // outfit_id: u8
        let _outfit_id = crate::util::read_u8_le(r)?;
        // outfit_id is expected to always be 0 (0)

        Ok(Self {
            name,
            race,
            class,
            gender,
            skin_color,
            face,
            hair_style,
            hair_color,
            facial_hair,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_CHAR_CREATE {}

impl CMSG_CHAR_CREATE {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 1 // race: Race
        + 1 // class: Class
        + 1 // gender: Gender
        + 1 // skin_color: u8
        + 1 // face: u8
        + 1 // hair_style: u8
        + 1 // hair_color: u8
        + 1 // facial_hair: u8
        + 1 // outfit_id: u8
    }
}

#[cfg(test)]
mod test {
    use super::CMSG_CHAR_CREATE;
    use crate::world::vanilla::Class;
    use crate::world::vanilla::Gender;
    use crate::world::vanilla::Race;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ClientOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 24] = [ 0x00, 0x16, 0x36, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61,
         0x64, 0x62, 0x65, 0x65, 0x66, 0x00, 0x01, 0x01, 0x01, 0x08, 0x00, 0x0E,
         0x02, 0x04, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm` line 18.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_CHAR_CREATE0() {
        let expected = CMSG_CHAR_CREATE {
            name: String::from("Deadbeef"),
            race: Race::Human,
            class: Class::Warrior,
            gender: Gender::Female,
            skin_color: 0x8,
            face: 0x0,
            hair_style: 0xE,
            hair_color: 0x2,
            facial_hair: 0x4,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_CREATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_CREATE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.name, expected.name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.class, expected.class);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.skin_color, expected.skin_color);
        assert_eq!(t.face, expected.face);
        assert_eq!(t.hair_style, expected.hair_style);
        assert_eq!(t.hair_color, expected.hair_color);
        assert_eq!(t.facial_hair, expected.facial_hair);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm` line 18.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_CHAR_CREATE0() {
        let expected = CMSG_CHAR_CREATE {
            name: String::from("Deadbeef"),
            race: Race::Human,
            class: Class::Warrior,
            gender: Gender::Female,
            skin_color: 0x8,
            face: 0x0,
            hair_style: 0xE,
            hair_color: 0x2,
            facial_hair: 0x4,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_CREATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_CREATE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.name, expected.name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.class, expected.class);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.skin_color, expected.skin_color);
        assert_eq!(t.face, expected.face);
        assert_eq!(t.hair_style, expected.hair_style);
        assert_eq!(t.hair_color, expected.hair_color);
        assert_eq!(t.facial_hair, expected.facial_hair);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm` line 18.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_CHAR_CREATE0() {
        let expected = CMSG_CHAR_CREATE {
            name: String::from("Deadbeef"),
            race: Race::Human,
            class: Class::Warrior,
            gender: Gender::Female,
            skin_color: 0x8,
            face: 0x0,
            hair_style: 0xE,
            hair_color: 0x2,
            facial_hair: 0x4,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_CREATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_CREATE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.name, expected.name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.class, expected.class);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.skin_color, expected.skin_color);
        assert_eq!(t.face, expected.face);
        assert_eq!(t.hair_style, expected.hair_style);
        assert_eq!(t.hair_color, expected.hair_color);
        assert_eq!(t.facial_hair, expected.facial_hair);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
