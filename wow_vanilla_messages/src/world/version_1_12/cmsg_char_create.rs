use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::class::{Class, class_try_from};
use crate::world::version_1_12::gender::{Gender, gender_try_from};
use crate::world::version_1_12::race::{Race, race_try_from};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Sent after the client presses 'Create Character'. The client will then wait for [`SMSG_CHAR_CREATE`](crate::world::version_1_12::SMSG_CHAR_CREATE).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm#L3):
/// ```text
/// cmsg CMSG_CHAR_CREATE = 0x0036 {
///     CString name;
///     Race race;
///     Class class;
///     Gender gender;
///     u8 skin;
///     u8 face;
///     u8 hairstyle;
///     u8 haircolor;
///     u8 facialhair;
///     u8 outfit_id = 0;
/// }
/// ```
pub struct CMSG_CHAR_CREATE {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub skin: u8,
    pub face: u8,
    pub hairstyle: u8,
    pub haircolor: u8,
    pub facialhair: u8,
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

impl ClientMessage for CMSG_CHAR_CREATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&(self.race.as_int() as u8).to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u8).to_le_bytes())?;

        // skin: u8
        w.write_all(&self.skin.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hairstyle: u8
        w.write_all(&self.hairstyle.to_le_bytes())?;

        // haircolor: u8
        w.write_all(&self.haircolor.to_le_bytes())?;

        // facialhair: u8
        w.write_all(&self.facialhair.to_le_bytes())?;

        // outfit_id: u8
        w.write_all(&Self::OUTFIT_ID_VALUE.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0036;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // race: Race
        let race: Race = race_try_from(crate::util::read_u8_le(r)?)?;

        // class: Class
        let class: Class = class_try_from(crate::util::read_u8_le(r)?)?;

        // gender: Gender
        let gender: Gender = gender_try_from(crate::util::read_u8_le(r)?)?;

        // skin: u8
        let skin = crate::util::read_u8_le(r)?;

        // face: u8
        let face = crate::util::read_u8_le(r)?;

        // hairstyle: u8
        let hairstyle = crate::util::read_u8_le(r)?;

        // haircolor: u8
        let haircolor = crate::util::read_u8_le(r)?;

        // facialhair: u8
        let facialhair = crate::util::read_u8_le(r)?;

        // outfit_id: u8
        let _outfit_id = crate::util::read_u8_le(r)?;
        // outfit_id is expected to always be 0 (0)

        Ok(Self {
            name,
            race,
            class,
            gender,
            skin,
            face,
            hairstyle,
            haircolor,
            facialhair,
        })
    }

}

impl CMSG_CHAR_CREATE {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 1 // race: Race
        + 1 // class: Class
        + 1 // gender: Gender
        + 1 // skin: u8
        + 1 // face: u8
        + 1 // hairstyle: u8
        + 1 // haircolor: u8
        + 1 // facialhair: u8
        + 1 // outfit_id: u8
    }
}

#[cfg(test)]
mod test {
    use super::CMSG_CHAR_CREATE;
    use crate::world::version_1_12::Class;
    use crate::world::version_1_12::Gender;
    use crate::world::version_1_12::Race;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 24] = [ 0x00, 0x16, 0x36, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61,
         0x64, 0x62, 0x65, 0x65, 0x66, 0x00, 0x01, 0x01, 0x01, 0x08, 0x00, 0x0E,
         0x02, 0x04, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm` line 18.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_CHAR_CREATE0() {
        let expected = CMSG_CHAR_CREATE {
            name: String::from("Deadbeef"),
            race: Race::HUMAN,
            class: Class::WARRIOR,
            gender: Gender::FEMALE,
            skin: 0x8,
            face: 0x0,
            hairstyle: 0xE,
            haircolor: 0x2,
            facialhair: 0x4,
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
        assert_eq!(t.skin, expected.skin);
        assert_eq!(t.face, expected.face);
        assert_eq!(t.hairstyle, expected.hairstyle);
        assert_eq!(t.haircolor, expected.haircolor);
        assert_eq!(t.facialhair, expected.facialhair);

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
            race: Race::HUMAN,
            class: Class::WARRIOR,
            gender: Gender::FEMALE,
            skin: 0x8,
            face: 0x0,
            hairstyle: 0xE,
            haircolor: 0x2,
            facialhair: 0x4,
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
        assert_eq!(t.skin, expected.skin);
        assert_eq!(t.face, expected.face);
        assert_eq!(t.hairstyle, expected.hairstyle);
        assert_eq!(t.haircolor, expected.haircolor);
        assert_eq!(t.facialhair, expected.facialhair);

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
            race: Race::HUMAN,
            class: Class::WARRIOR,
            gender: Gender::FEMALE,
            skin: 0x8,
            face: 0x0,
            hairstyle: 0xE,
            haircolor: 0x2,
            facialhair: 0x4,
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
        assert_eq!(t.skin, expected.skin);
        assert_eq!(t.face, expected.face);
        assert_eq!(t.hairstyle, expected.hairstyle);
        assert_eq!(t.haircolor, expected.haircolor);
        assert_eq!(t.facialhair, expected.facialhair);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
