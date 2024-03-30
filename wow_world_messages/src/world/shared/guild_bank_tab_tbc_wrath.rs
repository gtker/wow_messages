use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm#L15):
/// ```text
/// struct GuildBankTab {
///     CString tab_name;
///     CString tab_icon;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct GuildBankTab {
    pub tab_name: String,
    pub tab_icon: String,
}

impl GuildBankTab {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // tab_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.tab_name.as_bytes().iter().next_back(), Some(&0_u8), "String `tab_name` must not be null-terminated.");
        w.write_all(self.tab_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // tab_icon: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.tab_icon.as_bytes().iter().next_back(), Some(&0_u8), "String `tab_icon` must not be null-terminated.");
        w.write_all(self.tab_icon.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl GuildBankTab {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // tab_name: CString
        let tab_name = {
            let tab_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(tab_name)?
        };

        // tab_icon: CString
        let tab_icon = {
            let tab_icon = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(tab_icon)?
        };

        Ok(Self {
            tab_name,
            tab_icon,
        })
    }

}

impl GuildBankTab {
    pub(crate) fn size(&self) -> usize {
        self.tab_name.len() + 1 // tab_name: CString
        + self.tab_icon.len() + 1 // tab_icon: CString
    }
}

