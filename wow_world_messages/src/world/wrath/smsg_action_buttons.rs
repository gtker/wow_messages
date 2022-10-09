use std::convert::{TryFrom, TryInto};
use crate::world::wrath::ActionBarBehavior;
use crate::world::wrath::ActionButton;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm#L29):
/// ```text
/// smsg SMSG_ACTION_BUTTONS = 0x0129 {
///     ActionBarBehavior behavior;
///     if (behavior != CLEAR) {
///         ActionButton[136] data;
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // behavior: ActionBarBehavior
        w.write_all(&(self.behavior.as_int() as u8).to_le_bytes())?;

        match &self.behavior {
            SMSG_ACTION_BUTTONS_ActionBarBehavior::Initial {
                data,
            } => {
                // data: ActionButton[136]
                for i in data.iter() {
                    i.write_into_vec(w)?;
                }

            }
            SMSG_ACTION_BUTTONS_ActionBarBehavior::Set {
                data,
            } => {
                // data: ActionButton[136]
                for i in data.iter() {
                    i.write_into_vec(w)?;
                }

            }
            SMSG_ACTION_BUTTONS_ActionBarBehavior::Clear => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // behavior: ActionBarBehavior
        let behavior: ActionBarBehavior = crate::util::read_u8_le(r)?.try_into()?;

        let behavior_if = match behavior {
            ActionBarBehavior::Initial => {
                // data: ActionButton[136]
                let mut data = [ActionButton::default(); 136];
                for i in data.iter_mut() {
                    *i = ActionButton::read(r)?;
                }

                SMSG_ACTION_BUTTONS_ActionBarBehavior::Initial {
                    data,
                }
            }
            ActionBarBehavior::Set => {
                // data: ActionButton[136]
                let mut data = [ActionButton::default(); 136];
                for i in data.iter_mut() {
                    *i = ActionButton::read(r)?;
                }

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
impl crate::world::wrath::ServerMessage for SMSG_ACTION_BUTTONS {}

impl SMSG_ACTION_BUTTONS {
    pub(crate) fn size(&self) -> usize {
        self.behavior.size() // behavior: SMSG_ACTION_BUTTONS_ActionBarBehavior
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SMSG_ACTION_BUTTONS_ActionBarBehavior {
    Initial {
        data: [ActionButton; 136],
    },
    Set {
        data: [ActionButton; 136],
    },
    Clear,
}

impl Default for SMSG_ACTION_BUTTONS_ActionBarBehavior {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Initial {
            data: [Default::default(); 136],
        }
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
                + 136 * 4 // data: ActionButton[136]
            }
            Self::Set {
                data,
            } => {
                1
                + 136 * 4 // data: ActionButton[136]
            }
            Self::Clear => {
                1
            }
        }
    }
}

