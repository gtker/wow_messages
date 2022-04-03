use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm#L33):
/// ```text
/// struct Version {
///     u8 major;
///     u8 minor;
///     u8 patch;
///     u16 build;
/// }
/// ```
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u16,
}

impl ReadableAndWritable for Version {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // major: u8
        let major = crate::util::read_u8_le(r)?;

        // minor: u8
        let minor = crate::util::read_u8_le(r)?;

        // patch: u8
        let patch = crate::util::read_u8_le(r)?;

        // build: u16
        let build = crate::util::read_u16_le(r)?;

        Ok(Self {
            major,
            minor,
            patch,
            build,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // major: u8
        w.write_all(&self.major.to_le_bytes())?;

        // minor: u8
        w.write_all(&self.minor.to_le_bytes())?;

        // patch: u8
        w.write_all(&self.patch.to_le_bytes())?;

        // build: u16
        w.write_all(&self.build.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for Version {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for Version {
    fn maximum_possible_size() -> usize {
        1 // major: u8
        + 1 // minor: u8
        + 1 // patch: u8
        + 2 // build: u16
    }
}

