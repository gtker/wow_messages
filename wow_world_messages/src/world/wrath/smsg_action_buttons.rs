use std::io::{Read, Write};

use crate::wrath::{
    ActionBarBehavior,
    ActionButton,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm#L29):
/// ```text
/// smsg SMSG_ACTION_BUTTONS = 0x0129 {
///     ActionBarBehavior behavior;
///     if (behavior != CLEAR) {
///         ActionButton[144] data;
///     }
/// }
/// ```
pub struct SMSG_ACTION_BUTTONS {
    pub behavior: SMSG_ACTION_BUTTONS_ActionBarBehavior,
}

impl crate::Message for SMSG_ACTION_BUTTONS {
    const OPCODE: u32 = 0x0129;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // behavior: ActionBarBehavior
        w.write_all(&u8::from(self.behavior.as_int()).to_le_bytes())?;

        match &self.behavior {
            SMSG_ACTION_BUTTONS_ActionBarBehavior::Initial {
                data,
            } => {
                // data: ActionButton[144]
                for i in data.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
            SMSG_ACTION_BUTTONS_ActionBarBehavior::Set {
                data,
            } => {
                // data: ActionButton[144]
                for i in data.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
            SMSG_ACTION_BUTTONS_ActionBarBehavior::Clear => {}
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=577).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0129, size: body_size as u32 });
        }

        // behavior: ActionBarBehavior
        let behavior: ActionBarBehavior = crate::util::read_u8_le(&mut r)?.try_into()?;

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

                SMSG_ACTION_BUTTONS_ActionBarBehavior::Initial {
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

                SMSG_ACTION_BUTTONS_ActionBarBehavior::Set {
                    data,
                }
            }
            ActionBarBehavior::Clear => SMSG_ACTION_BUTTONS_ActionBarBehavior::Clear,
        };

        Ok(Self {
            behavior: behavior_if,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACTION_BUTTONS {}

impl SMSG_ACTION_BUTTONS {
    pub(crate) fn size(&self) -> usize {
        self.behavior.size() // behavior: SMSG_ACTION_BUTTONS_ActionBarBehavior
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_ACTION_BUTTONS_ActionBarBehavior {
    Initial {
        data: [ActionButton; 144],
    },
    Set {
        data: [ActionButton; 144],
    },
    Clear,
}

impl Default for SMSG_ACTION_BUTTONS_ActionBarBehavior {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Clear
    }
}

impl SMSG_ACTION_BUTTONS_ActionBarBehavior {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Initial { .. } => 0,
            Self::Set { .. } => 1,
            Self::Clear => 2,
        }
    }

}

impl SMSG_ACTION_BUTTONS_ActionBarBehavior {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Initial {
                data,
            } => {
                1
                + 144 * 4 // data: ActionButton[144]
            }
            Self::Set {
                data,
            } => {
                1
                + 144 * 4 // data: ActionButton[144]
            }
            Self::Clear => {
                1
            }
        }
    }
}

