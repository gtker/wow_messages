use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Class, ClassError};
use crate::world::v1::v12::{Gender, GenderError};
use crate::world::v1::v12::{Race, RaceError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_NAME_QUERY_RESPONSE {
    pub guid: Guid,
    pub character_name: String,
    pub realm_name: String,
    pub race: Race,
    pub gender: Gender,
    pub class: Class,
}

impl ServerMessageWrite for SMSG_NAME_QUERY_RESPONSE {}

impl MessageBody for SMSG_NAME_QUERY_RESPONSE {
    const OPCODE: u16 = 0x0051;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_NAME_QUERY_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // character_name: CString
        let character_name = crate::util::read_c_string_to_vec(r)?;
        let character_name = String::from_utf8(character_name)?;

        // realm_name: CString
        let realm_name = crate::util::read_c_string_to_vec(r)?;
        let realm_name = String::from_utf8(realm_name)?;

        // race: Race
        let race: Race = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // gender: Gender
        let gender: Gender = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // class: Class
        let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            guid,
            character_name,
            realm_name,
            race,
            gender,
            class,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // character_name: CString
        w.write_all(self.character_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // realm_name: CString
        w.write_all(self.realm_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        crate::util::write_u32_le(w, self.race.as_int() as u32)?;

        // gender: Gender
        crate::util::write_u32_le(w, self.gender.as_int() as u32)?;

        // class: Class
        crate::util::write_u32_le(w, self.class.as_int() as u32)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // character_name: CString
            let character_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let character_name = String::from_utf8(character_name)?;

            // realm_name: CString
            let realm_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let realm_name = String::from_utf8(realm_name)?;

            // race: Race
            let race: Race = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            // gender: Gender
            let gender: Gender = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            // class: Class
            let class: Class = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                guid,
                character_name,
                realm_name,
                race,
                gender,
                class,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // character_name: CString
            w.write_all(self.character_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // realm_name: CString
            w.write_all(self.realm_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // race: Race
            crate::util::tokio_write_u32_le(w, self.race.as_int() as u32).await?;

            // gender: Gender
            crate::util::tokio_write_u32_le(w, self.gender.as_int() as u32).await?;

            // class: Class
            crate::util::tokio_write_u32_le(w, self.class.as_int() as u32).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // character_name: CString
            let character_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let character_name = String::from_utf8(character_name)?;

            // realm_name: CString
            let realm_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let realm_name = String::from_utf8(realm_name)?;

            // race: Race
            let race: Race = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            // gender: Gender
            let gender: Gender = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            // class: Class
            let class: Class = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                guid,
                character_name,
                realm_name,
                race,
                gender,
                class,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.astd_write(w).await?;

            // character_name: CString
            w.write_all(self.character_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // realm_name: CString
            w.write_all(self.realm_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // race: Race
            crate::util::astd_write_u32_le(w, self.race.as_int() as u32).await?;

            // gender: Gender
            crate::util::astd_write_u32_le(w, self.gender.as_int() as u32).await?;

            // class: Class
            crate::util::astd_write_u32_le(w, self.class.as_int() as u32).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_NAME_QUERY_RESPONSE {
    fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + self.character_name.len() + 1 // character_name: CString
        + self.realm_name.len() + 1 // realm_name: CString
        + 4 // race: Race
        + 4 // gender: Gender
        + 4 // class: Class
    }
}

impl MaximumPossibleSized for SMSG_NAME_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 256 // character_name: CString
        + 256 // realm_name: CString
        + 1 // race: Race
        + 1 // gender: Gender
        + 1 // class: Class
    }
}

#[derive(Debug)]
pub enum SMSG_NAME_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Class(ClassError),
    Gender(GenderError),
    Race(RaceError),
}

impl std::error::Error for SMSG_NAME_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_NAME_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Class(i) => i.fmt(f),
            Self::Gender(i) => i.fmt(f),
            Self::Race(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ClassError> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e: ClassError) -> Self {
        Self::Class(e)
    }
}

impl From<GenderError> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e: GenderError) -> Self {
        Self::Gender(e)
    }
}

impl From<RaceError> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e: RaceError) -> Self {
        Self::Race(e)
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_NAME_QUERY_RESPONSE;
    use crate::VariableSized;
    use crate::world::v1::v12::Class;
    use crate::world::v1::v12::Gender;
    use crate::world::v1::v12::Race;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ServerOpcodeMessage;
    use crate::{MessageBody, ClientMessageWrite, ServerMessageWrite, OpcodeMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_NAME_QUERY_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x1C, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x00, 0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x00, 0x01, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from(""),
            race: Race::HUMAN,
            gender: Gender::MALE,
            class: Class::WARRIOR,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_SMSG_NAME_QUERY_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x1C, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x00, 0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x00, 0x01, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from(""),
            race: Race::HUMAN,
            gender: Gender::MALE,
            class: Class::WARRIOR,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_SMSG_NAME_QUERY_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x1C, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x00, 0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x00, 0x01, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from(""),
            race: Race::HUMAN,
            gender: Gender::MALE,
            class: Class::WARRIOR,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_NAME_QUERY_RESPONSE1() {
        let raw: Vec<u8> = vec![ 0x00, 0x1D, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x00, 0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x41, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from("A"),
            race: Race::HUMAN,
            gender: Gender::MALE,
            class: Class::WARRIOR,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", tokio::test)]
    async fn tokio_SMSG_NAME_QUERY_RESPONSE1() {
        let raw: Vec<u8> = vec![ 0x00, 0x1D, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x00, 0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x41, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from("A"),
            race: Race::HUMAN,
            gender: Gender::MALE,
            class: Class::WARRIOR,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", async_std::test)]
    async fn astd_SMSG_NAME_QUERY_RESPONSE1() {
        let raw: Vec<u8> = vec![ 0x00, 0x1D, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x00, 0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x41, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from("A"),
            race: Race::HUMAN,
            gender: Gender::MALE,
            class: Class::WARRIOR,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
