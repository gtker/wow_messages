use std::io::{Read, Write};

use crate::wrath::{
    ActionBarBehavior, ActionButton,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm#L29):
/// ```text
/// smsg SMSG_ACTION_BUTTONS = 0x0129 {
///     ActionBarBehavior behavior;
///     if (behavior != CLEAR) {
///         ActionButton[144] data;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_ACTION_BUTTONS {
    Initial {
        data: [ActionButton; 144],
    },
    Set {
        data: [ActionButton; 144],
    },
    Clear,
}

impl crate::private::Sealed for SMSG_ACTION_BUTTONS {}
impl SMSG_ACTION_BUTTONS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=577).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // behavior: ActionBarBehavior
        let behavior = crate::util::read_u8_le(&mut r)?.try_into()?;

        let behavior_if = match behavior {
            ActionBarBehavior::Initial => {
                // data: ActionButton[144]
                let data = {
                    let mut data = [ActionButton::default(); 144];
                    for i in data.iter_mut() {
                        *i = ActionButton::read(&mut r)?;
                    }
                    data
                };

                SMSG_ACTION_BUTTONS::Initial {
                    data,
                }
            }
            ActionBarBehavior::Set => {
                // data: ActionButton[144]
                let data = {
                    let mut data = [ActionButton::default(); 144];
                    for i in data.iter_mut() {
                        *i = ActionButton::read(&mut r)?;
                    }
                    data
                };

                SMSG_ACTION_BUTTONS::Set {
                    data,
                }
            }
            ActionBarBehavior::Clear => SMSG_ACTION_BUTTONS::Clear,
        };

        Ok(behavior_if)
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
        writeln!(s, "    behavior = {};", ActionBarBehavior::try_from(self.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self {
            crate::wrath::SMSG_ACTION_BUTTONS::Initial {
                data,
            } => {
                writeln!(s, "    data = [").unwrap();
                for v in data.as_slice() {
                    writeln!(s, "        {{").unwrap();
                    // Members
                    writeln!(s, "            action = {};", v.action).unwrap();
                    writeln!(s, "            action_type = {};", v.action_type).unwrap();
                    writeln!(s, "            misc = {};", v.misc).unwrap();

                    writeln!(s, "        }},").unwrap();
                }
                writeln!(s, "    ];").unwrap();
            }
            crate::wrath::SMSG_ACTION_BUTTONS::Set {
                data,
            } => {
                writeln!(s, "    data = [").unwrap();
                for v in data.as_slice() {
                    writeln!(s, "        {{").unwrap();
                    // Members
                    writeln!(s, "            action = {};", v.action).unwrap();
                    writeln!(s, "            action_type = {};", v.action_type).unwrap();
                    writeln!(s, "            misc = {};", v.misc).unwrap();

                    writeln!(s, "        }},").unwrap();
                }
                writeln!(s, "    ];").unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 297_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "behavior", "    ");
        match &self {
            crate::wrath::SMSG_ACTION_BUTTONS::Initial {
                data,
            } => {
                writeln!(s, "    /* data: ActionButton[144] start */").unwrap();
                for (i, v) in data.iter().enumerate() {
                    writeln!(s, "    /* data: ActionButton[144] {i} start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 2, "action", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "action_type", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "misc", "        ");
                    writeln!(s, "    /* data: ActionButton[144] {i} end */").unwrap();
                }
                writeln!(s, "    /* data: ActionButton[144] end */").unwrap();
            }
            crate::wrath::SMSG_ACTION_BUTTONS::Set {
                data,
            } => {
                writeln!(s, "    /* data: ActionButton[144] start */").unwrap();
                for (i, v) in data.iter().enumerate() {
                    writeln!(s, "    /* data: ActionButton[144] {i} start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 2, "action", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "action_type", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "misc", "        ");
                    writeln!(s, "    /* data: ActionButton[144] {i} end */").unwrap();
                }
                writeln!(s, "    /* data: ActionButton[144] end */").unwrap();
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // behavior: ActionBarBehavior
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            SMSG_ACTION_BUTTONS::Initial {
                data,
            } => {
                // data: ActionButton[144]
                for i in data.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
            SMSG_ACTION_BUTTONS::Set {
                data,
            } => {
                // data: ActionButton[144]
                for i in data.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(297, "SMSG_ACTION_BUTTONS", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACTION_BUTTONS {}

impl SMSG_ACTION_BUTTONS {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::Initial {
                ..
            } => {
                1
                + 144 * 4 // data: ActionButton[144]
            }
            Self::Set {
                ..
            } => {
                1
                + 144 * 4 // data: ActionButton[144]
            }
            _ => 1,
        }) // behavior: SMSG_ACTION_BUTTONS
    }
}

impl Default for SMSG_ACTION_BUTTONS {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Clear
    }
}

impl SMSG_ACTION_BUTTONS {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Initial { .. } => 0,
            Self::Set { .. } => 1,
            Self::Clear => 2,
        }
    }

}

impl std::fmt::Display for SMSG_ACTION_BUTTONS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initial{ .. } => f.write_str("Initial"),
            Self::Set{ .. } => f.write_str("Set"),
            Self::Clear => f.write_str("Clear"),
        }
    }
}

