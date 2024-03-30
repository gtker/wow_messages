use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm:102`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm#L102):
/// ```text
/// struct MailListItemEnchant {
///     u32 charges;
///     u32 duration;
///     u32 enchant_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MailListItemEnchant {
    pub charges: u32,
    pub duration: u32,
    pub enchant_id: u32,
}

impl MailListItemEnchant {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // charges: u32
        w.write_all(&self.charges.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // enchant_id: u32
        w.write_all(&self.enchant_id.to_le_bytes())?;

        Ok(())
    }
}

impl MailListItemEnchant {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // charges: u32
        let charges = crate::util::read_u32_le(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        // enchant_id: u32
        let enchant_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            charges,
            duration,
            enchant_id,
        })
    }

}

