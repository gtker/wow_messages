use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm#L1):
/// ```text
/// smsg SMSG_ACTION_BUTTONS = 0x0129 {
///     u32[120] data;
/// }
/// ```
pub struct SMSG_ACTION_BUTTONS {
    pub data: [u32; 120],
}

impl crate::private::Sealed for SMSG_ACTION_BUTTONS {}
impl SMSG_ACTION_BUTTONS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 480 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // data: u32[120]
        let data = {
            let mut data = [u32::default(); 120];
            for i in data.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            data
        };

        Ok(Self {
            data,
        })
    }

}

impl crate::Message for SMSG_ACTION_BUTTONS {
    const OPCODE: u32 = 0x0129;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ACTION_BUTTONS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ACTION_BUTTONS {{").unwrap();
        // Members
        writeln!(s, "    data = [").unwrap();
        for v in self.data.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 482_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 297_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    /* data: u32[120] start */").unwrap();
        for (i, v) in self.data.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("data {i}"), "    ");
        }
        writeln!(s, "    /* data: u32[120] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        480
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // data: u32[120]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(297, "SMSG_ACTION_BUTTONS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ACTION_BUTTONS {}

