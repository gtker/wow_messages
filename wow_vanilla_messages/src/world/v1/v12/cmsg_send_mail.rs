use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_SEND_MAIL {
    pub mailbox: Guid,
    pub receiver: String,
    pub subject: String,
    pub body: String,
    pub unknown1: u32,
    pub unknown2: u32,
    pub item: Guid,
    pub money: u32,
    pub cash_on_delivery_amount: u32,
    pub unknown3: u32,
    pub unknown4: u32,
}

impl ClientMessageWrite for CMSG_SEND_MAIL {}

impl MessageBody for CMSG_SEND_MAIL {
    const OPCODE: u16 = 0x0238;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_SEND_MAILError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // mailbox: Guid
        let mailbox = Guid::read(r)?;

        // receiver: CString
        let receiver = crate::util::read_c_string_to_vec(r)?;
        let receiver = String::from_utf8(receiver)?;

        // subject: CString
        let subject = crate::util::read_c_string_to_vec(r)?;
        let subject = String::from_utf8(subject)?;

        // body: CString
        let body = crate::util::read_c_string_to_vec(r)?;
        let body = String::from_utf8(body)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        // money: u32
        let money = crate::util::read_u32_le(r)?;

        // cash_on_delivery_amount: u32
        let cash_on_delivery_amount = crate::util::read_u32_le(r)?;

        // unknown3: u32
        let unknown3 = crate::util::read_u32_le(r)?;

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(r)?;

        Ok(Self {
            mailbox,
            receiver,
            subject,
            body,
            unknown1,
            unknown2,
            item,
            money,
            cash_on_delivery_amount,
            unknown3,
            unknown4,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // mailbox: Guid
        self.mailbox.write(w)?;

        // receiver: CString
        w.write_all(self.receiver.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // subject: CString
        w.write_all(self.subject.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body: CString
        w.write_all(self.body.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // item: Guid
        self.item.write(w)?;

        // money: u32
        w.write_all(&self.money.to_le_bytes())?;

        // cash_on_delivery_amount: u32
        w.write_all(&self.cash_on_delivery_amount.to_le_bytes())?;

        // unknown3: u32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "tokio")]
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
            // mailbox: Guid
            let mailbox = Guid::tokio_read(r).await?;

            // receiver: CString
            let receiver = crate::util::tokio_read_c_string_to_vec(r).await?;
            let receiver = String::from_utf8(receiver)?;

            // subject: CString
            let subject = crate::util::tokio_read_c_string_to_vec(r).await?;
            let subject = String::from_utf8(subject)?;

            // body: CString
            let body = crate::util::tokio_read_c_string_to_vec(r).await?;
            let body = String::from_utf8(body)?;

            // unknown1: u32
            let unknown1 = crate::util::tokio_read_u32_le(r).await?;

            // unknown2: u32
            let unknown2 = crate::util::tokio_read_u32_le(r).await?;

            // item: Guid
            let item = Guid::tokio_read(r).await?;

            // money: u32
            let money = crate::util::tokio_read_u32_le(r).await?;

            // cash_on_delivery_amount: u32
            let cash_on_delivery_amount = crate::util::tokio_read_u32_le(r).await?;

            // unknown3: u32
            let unknown3 = crate::util::tokio_read_u32_le(r).await?;

            // unknown4: u32
            let unknown4 = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                mailbox,
                receiver,
                subject,
                body,
                unknown1,
                unknown2,
                item,
                money,
                cash_on_delivery_amount,
                unknown3,
                unknown4,
            })
        })
    }

    #[cfg(feature = "tokio")]
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
            // mailbox: Guid
            self.mailbox.tokio_write(w).await?;

            // receiver: CString
            w.write_all(self.receiver.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // subject: CString
            w.write_all(self.subject.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // body: CString
            w.write_all(self.body.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // unknown1: u32
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // unknown2: u32
            w.write_all(&self.unknown2.to_le_bytes()).await?;

            // item: Guid
            self.item.tokio_write(w).await?;

            // money: u32
            w.write_all(&self.money.to_le_bytes()).await?;

            // cash_on_delivery_amount: u32
            w.write_all(&self.cash_on_delivery_amount.to_le_bytes()).await?;

            // unknown3: u32
            w.write_all(&self.unknown3.to_le_bytes()).await?;

            // unknown4: u32
            w.write_all(&self.unknown4.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
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
            // mailbox: Guid
            let mailbox = Guid::astd_read(r).await?;

            // receiver: CString
            let receiver = crate::util::astd_read_c_string_to_vec(r).await?;
            let receiver = String::from_utf8(receiver)?;

            // subject: CString
            let subject = crate::util::astd_read_c_string_to_vec(r).await?;
            let subject = String::from_utf8(subject)?;

            // body: CString
            let body = crate::util::astd_read_c_string_to_vec(r).await?;
            let body = String::from_utf8(body)?;

            // unknown1: u32
            let unknown1 = crate::util::astd_read_u32_le(r).await?;

            // unknown2: u32
            let unknown2 = crate::util::astd_read_u32_le(r).await?;

            // item: Guid
            let item = Guid::astd_read(r).await?;

            // money: u32
            let money = crate::util::astd_read_u32_le(r).await?;

            // cash_on_delivery_amount: u32
            let cash_on_delivery_amount = crate::util::astd_read_u32_le(r).await?;

            // unknown3: u32
            let unknown3 = crate::util::astd_read_u32_le(r).await?;

            // unknown4: u32
            let unknown4 = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                mailbox,
                receiver,
                subject,
                body,
                unknown1,
                unknown2,
                item,
                money,
                cash_on_delivery_amount,
                unknown3,
                unknown4,
            })
        })
    }

    #[cfg(feature = "async-std")]
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
            // mailbox: Guid
            self.mailbox.astd_write(w).await?;

            // receiver: CString
            w.write_all(self.receiver.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // subject: CString
            w.write_all(self.subject.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // body: CString
            w.write_all(self.body.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // unknown1: u32
            w.write_all(&self.unknown1.to_le_bytes()).await?;

            // unknown2: u32
            w.write_all(&self.unknown2.to_le_bytes()).await?;

            // item: Guid
            self.item.astd_write(w).await?;

            // money: u32
            w.write_all(&self.money.to_le_bytes()).await?;

            // cash_on_delivery_amount: u32
            w.write_all(&self.cash_on_delivery_amount.to_le_bytes()).await?;

            // unknown3: u32
            w.write_all(&self.unknown3.to_le_bytes()).await?;

            // unknown4: u32
            w.write_all(&self.unknown4.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl CMSG_SEND_MAIL {
    pub fn size(&self) -> usize {
        0
        + 8 // mailbox: Guid
        + self.receiver.len() + 1 // receiver: CString
        + self.subject.len() + 1 // subject: CString
        + self.body.len() + 1 // body: CString
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 8 // item: Guid
        + 4 // money: u32
        + 4 // cash_on_delivery_amount: u32
        + 4 // unknown3: u32
        + 4 // unknown4: u32
    }
}

#[derive(Debug)]
pub enum CMSG_SEND_MAILError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_SEND_MAILError {}
impl std::fmt::Display for CMSG_SEND_MAILError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_SEND_MAILError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_SEND_MAILError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

