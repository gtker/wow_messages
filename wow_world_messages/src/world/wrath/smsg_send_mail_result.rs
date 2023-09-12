use std::io::{Read, Write};

use crate::wrath::{
    MailAction, MailResult, MailResultTwo,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_send_mail_result.wowm:51`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_send_mail_result.wowm#L51):
/// ```text
/// smsg SMSG_SEND_MAIL_RESULT = 0x0239 {
///     u32 mail_id;
///     MailAction action;
///     if (action == ITEM_TAKEN) {
///         MailResult result;
///         if (result == ERR_EQUIP_ERROR) {
///             u32 equip_error;
///         }
///         else {
///             u32 item;
///             u32 item_count;
///         }
///     }
///     else {
///         MailResultTwo result2;
///         if (result2 == ERR_EQUIP_ERROR) {
///             u32 equip_error2;
///         }
///     }
/// }
/// ```
pub struct SMSG_SEND_MAIL_RESULT {
    pub mail_id: u32,
    pub action: SMSG_SEND_MAIL_RESULT_MailAction,
}

impl crate::private::Sealed for SMSG_SEND_MAIL_RESULT {}
impl SMSG_SEND_MAIL_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(12..=20).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(&mut r)?;

        // action: MailAction
        let action = crate::util::read_u32_le(&mut r)?.try_into()?;

        let action_if = match action {
            MailAction::Send => {
                // result2: MailResultTwo
                let result2 = crate::util::read_u32_le(&mut r)?.try_into()?;

                let result2_if = match result2 {
                    MailResultTwo::Ok => SMSG_SEND_MAIL_RESULT_MailResultTwo::Ok,
                    MailResultTwo::ErrEquipError => {
                        // equip_error2: u32
                        let equip_error2 = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                            equip_error2,
                        }
                    }
                    MailResultTwo::ErrCannotSendToSelf => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCannotSendToSelf,
                    MailResultTwo::ErrNotEnoughMoney => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotEnoughMoney,
                    MailResultTwo::ErrRecipientNotFound => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientNotFound,
                    MailResultTwo::ErrNotYourTeam => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotYourTeam,
                    MailResultTwo::ErrInternalError => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrInternalError,
                    MailResultTwo::ErrDisabledForTrialAcc => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrDisabledForTrialAcc,
                    MailResultTwo::ErrRecipientCapReached => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientCapReached,
                    MailResultTwo::ErrCantSendWrappedCod => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCantSendWrappedCod,
                    MailResultTwo::ErrMailAndChatSuspended => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAndChatSuspended,
                    MailResultTwo::ErrTooManyAttachments => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrTooManyAttachments,
                    MailResultTwo::ErrMailAttachmentInvalid => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAttachmentInvalid,
                    MailResultTwo::ErrItemHasExpired => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrItemHasExpired,
                };

                SMSG_SEND_MAIL_RESULT_MailAction::Send {
                    result2: result2_if,
                }
            }
            MailAction::MoneyTaken => {
                // result2: MailResultTwo
                let result2 = crate::util::read_u32_le(&mut r)?.try_into()?;

                let result2_if = match result2 {
                    MailResultTwo::Ok => SMSG_SEND_MAIL_RESULT_MailResultTwo::Ok,
                    MailResultTwo::ErrEquipError => {
                        // equip_error2: u32
                        let equip_error2 = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                            equip_error2,
                        }
                    }
                    MailResultTwo::ErrCannotSendToSelf => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCannotSendToSelf,
                    MailResultTwo::ErrNotEnoughMoney => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotEnoughMoney,
                    MailResultTwo::ErrRecipientNotFound => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientNotFound,
                    MailResultTwo::ErrNotYourTeam => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotYourTeam,
                    MailResultTwo::ErrInternalError => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrInternalError,
                    MailResultTwo::ErrDisabledForTrialAcc => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrDisabledForTrialAcc,
                    MailResultTwo::ErrRecipientCapReached => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientCapReached,
                    MailResultTwo::ErrCantSendWrappedCod => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCantSendWrappedCod,
                    MailResultTwo::ErrMailAndChatSuspended => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAndChatSuspended,
                    MailResultTwo::ErrTooManyAttachments => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrTooManyAttachments,
                    MailResultTwo::ErrMailAttachmentInvalid => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAttachmentInvalid,
                    MailResultTwo::ErrItemHasExpired => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrItemHasExpired,
                };

                SMSG_SEND_MAIL_RESULT_MailAction::MoneyTaken {
                    result2: result2_if,
                }
            }
            MailAction::ItemTaken => {
                // result: MailResult
                let result = crate::util::read_u32_le(&mut r)?.try_into()?;

                let result_if = match result {
                    MailResult::Ok => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::Ok {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrEquipError => {
                        // equip_error: u32
                        let equip_error = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrEquipError {
                            equip_error,
                        }
                    }
                    MailResult::ErrCannotSendToSelf => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrCannotSendToSelf {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrNotEnoughMoney => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrNotEnoughMoney {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrRecipientNotFound => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrRecipientNotFound {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrNotYourTeam => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrNotYourTeam {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrInternalError => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrInternalError {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrDisabledForTrialAcc => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrDisabledForTrialAcc {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrRecipientCapReached => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrRecipientCapReached {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrCantSendWrappedCod => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrCantSendWrappedCod {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrMailAndChatSuspended => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrMailAndChatSuspended {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrTooManyAttachments => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrTooManyAttachments {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrMailAttachmentInvalid => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrMailAttachmentInvalid {
                            item,
                            item_count,
                        }
                    }
                    MailResult::ErrItemHasExpired => {
                        // item: u32
                        let item = crate::util::read_u32_le(&mut r)?;

                        // item_count: u32
                        let item_count = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResult::ErrItemHasExpired {
                            item,
                            item_count,
                        }
                    }
                };

                SMSG_SEND_MAIL_RESULT_MailAction::ItemTaken {
                    result: result_if,
                }
            }
            MailAction::ReturnedToSender => {
                // result2: MailResultTwo
                let result2 = crate::util::read_u32_le(&mut r)?.try_into()?;

                let result2_if = match result2 {
                    MailResultTwo::Ok => SMSG_SEND_MAIL_RESULT_MailResultTwo::Ok,
                    MailResultTwo::ErrEquipError => {
                        // equip_error2: u32
                        let equip_error2 = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                            equip_error2,
                        }
                    }
                    MailResultTwo::ErrCannotSendToSelf => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCannotSendToSelf,
                    MailResultTwo::ErrNotEnoughMoney => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotEnoughMoney,
                    MailResultTwo::ErrRecipientNotFound => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientNotFound,
                    MailResultTwo::ErrNotYourTeam => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotYourTeam,
                    MailResultTwo::ErrInternalError => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrInternalError,
                    MailResultTwo::ErrDisabledForTrialAcc => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrDisabledForTrialAcc,
                    MailResultTwo::ErrRecipientCapReached => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientCapReached,
                    MailResultTwo::ErrCantSendWrappedCod => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCantSendWrappedCod,
                    MailResultTwo::ErrMailAndChatSuspended => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAndChatSuspended,
                    MailResultTwo::ErrTooManyAttachments => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrTooManyAttachments,
                    MailResultTwo::ErrMailAttachmentInvalid => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAttachmentInvalid,
                    MailResultTwo::ErrItemHasExpired => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrItemHasExpired,
                };

                SMSG_SEND_MAIL_RESULT_MailAction::ReturnedToSender {
                    result2: result2_if,
                }
            }
            MailAction::Deleted => {
                // result2: MailResultTwo
                let result2 = crate::util::read_u32_le(&mut r)?.try_into()?;

                let result2_if = match result2 {
                    MailResultTwo::Ok => SMSG_SEND_MAIL_RESULT_MailResultTwo::Ok,
                    MailResultTwo::ErrEquipError => {
                        // equip_error2: u32
                        let equip_error2 = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                            equip_error2,
                        }
                    }
                    MailResultTwo::ErrCannotSendToSelf => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCannotSendToSelf,
                    MailResultTwo::ErrNotEnoughMoney => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotEnoughMoney,
                    MailResultTwo::ErrRecipientNotFound => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientNotFound,
                    MailResultTwo::ErrNotYourTeam => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotYourTeam,
                    MailResultTwo::ErrInternalError => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrInternalError,
                    MailResultTwo::ErrDisabledForTrialAcc => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrDisabledForTrialAcc,
                    MailResultTwo::ErrRecipientCapReached => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientCapReached,
                    MailResultTwo::ErrCantSendWrappedCod => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCantSendWrappedCod,
                    MailResultTwo::ErrMailAndChatSuspended => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAndChatSuspended,
                    MailResultTwo::ErrTooManyAttachments => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrTooManyAttachments,
                    MailResultTwo::ErrMailAttachmentInvalid => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAttachmentInvalid,
                    MailResultTwo::ErrItemHasExpired => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrItemHasExpired,
                };

                SMSG_SEND_MAIL_RESULT_MailAction::Deleted {
                    result2: result2_if,
                }
            }
            MailAction::MadePermanent => {
                // result2: MailResultTwo
                let result2 = crate::util::read_u32_le(&mut r)?.try_into()?;

                let result2_if = match result2 {
                    MailResultTwo::Ok => SMSG_SEND_MAIL_RESULT_MailResultTwo::Ok,
                    MailResultTwo::ErrEquipError => {
                        // equip_error2: u32
                        let equip_error2 = crate::util::read_u32_le(&mut r)?;

                        SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                            equip_error2,
                        }
                    }
                    MailResultTwo::ErrCannotSendToSelf => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCannotSendToSelf,
                    MailResultTwo::ErrNotEnoughMoney => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotEnoughMoney,
                    MailResultTwo::ErrRecipientNotFound => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientNotFound,
                    MailResultTwo::ErrNotYourTeam => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrNotYourTeam,
                    MailResultTwo::ErrInternalError => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrInternalError,
                    MailResultTwo::ErrDisabledForTrialAcc => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrDisabledForTrialAcc,
                    MailResultTwo::ErrRecipientCapReached => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrRecipientCapReached,
                    MailResultTwo::ErrCantSendWrappedCod => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrCantSendWrappedCod,
                    MailResultTwo::ErrMailAndChatSuspended => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAndChatSuspended,
                    MailResultTwo::ErrTooManyAttachments => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrTooManyAttachments,
                    MailResultTwo::ErrMailAttachmentInvalid => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrMailAttachmentInvalid,
                    MailResultTwo::ErrItemHasExpired => SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrItemHasExpired,
                };

                SMSG_SEND_MAIL_RESULT_MailAction::MadePermanent {
                    result2: result2_if,
                }
            }
        };

        Ok(Self {
            mail_id,
            action: action_if,
        })
    }

}

impl crate::Message for SMSG_SEND_MAIL_RESULT {
    const OPCODE: u32 = 0x0239;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SEND_MAIL_RESULT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SEND_MAIL_RESULT {{").unwrap();
        // Members
        writeln!(s, "    mail_id = {};", self.mail_id).unwrap();
        writeln!(s, "    action = {};", MailAction::try_from(self.action.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.action {
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::Send {
                result2,
            } => {
                writeln!(s, "    result2 = {};", MailResultTwo::try_from(result2.as_int()).unwrap().as_test_case_value()).unwrap();
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        writeln!(s, "    equip_error2 = {};", equip_error2).unwrap();
                    }
                    _ => {}
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::MoneyTaken {
                result2,
            } => {
                writeln!(s, "    result2 = {};", MailResultTwo::try_from(result2.as_int()).unwrap().as_test_case_value()).unwrap();
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        writeln!(s, "    equip_error2 = {};", equip_error2).unwrap();
                    }
                    _ => {}
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::ItemTaken {
                result,
            } => {
                writeln!(s, "    result = {};", MailResult::try_from(result.as_int()).unwrap().as_test_case_value()).unwrap();
                match &result {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::Ok {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrEquipError {
                        equip_error,
                    } => {
                        writeln!(s, "    equip_error = {};", equip_error).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrCannotSendToSelf {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrNotEnoughMoney {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrRecipientNotFound {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrNotYourTeam {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrInternalError {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrDisabledForTrialAcc {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrRecipientCapReached {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrCantSendWrappedCod {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrMailAndChatSuspended {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrTooManyAttachments {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrMailAttachmentInvalid {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrItemHasExpired {
                        item,
                        item_count,
                    } => {
                        writeln!(s, "    item = {};", item).unwrap();
                        writeln!(s, "    item_count = {};", item_count).unwrap();
                    }
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::ReturnedToSender {
                result2,
            } => {
                writeln!(s, "    result2 = {};", MailResultTwo::try_from(result2.as_int()).unwrap().as_test_case_value()).unwrap();
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        writeln!(s, "    equip_error2 = {};", equip_error2).unwrap();
                    }
                    _ => {}
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::Deleted {
                result2,
            } => {
                writeln!(s, "    result2 = {};", MailResultTwo::try_from(result2.as_int()).unwrap().as_test_case_value()).unwrap();
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        writeln!(s, "    equip_error2 = {};", equip_error2).unwrap();
                    }
                    _ => {}
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::MadePermanent {
                result2,
            } => {
                writeln!(s, "    result2 = {};", MailResultTwo::try_from(result2.as_int()).unwrap().as_test_case_value()).unwrap();
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        writeln!(s, "    equip_error2 = {};", equip_error2).unwrap();
                    }
                    _ => {}
                }

            }
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 569_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "mail_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "action", "    ");
        match &self.action {
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::Send {
                result2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "result2", "    ");
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "equip_error2", "    ");
                    }
                    _ => {}
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::MoneyTaken {
                result2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "result2", "    ");
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "equip_error2", "    ");
                    }
                    _ => {}
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::ItemTaken {
                result,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "result", "    ");
                match &result {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::Ok {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrEquipError {
                        equip_error,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "equip_error", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrCannotSendToSelf {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrNotEnoughMoney {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrRecipientNotFound {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrNotYourTeam {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrInternalError {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrDisabledForTrialAcc {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrRecipientCapReached {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrCantSendWrappedCod {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrMailAndChatSuspended {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrTooManyAttachments {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrMailAttachmentInvalid {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResult::ErrItemHasExpired {
                        item,
                        item_count,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "    ");
                    }
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::ReturnedToSender {
                result2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "result2", "    ");
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "equip_error2", "    ");
                    }
                    _ => {}
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::Deleted {
                result2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "result2", "    ");
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "equip_error2", "    ");
                    }
                    _ => {}
                }

            }
            crate::wrath::SMSG_SEND_MAIL_RESULT_MailAction::MadePermanent {
                result2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "result2", "    ");
                match &result2 {
                    crate::wrath::SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "equip_error2", "    ");
                    }
                    _ => {}
                }

            }
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
        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // action: MailAction
        w.write_all(&(self.action.as_int().to_le_bytes()))?;

        match &self.action {
            SMSG_SEND_MAIL_RESULT_MailAction::Send {
                result2,
            } => {
                // result2: MailResultTwo
                w.write_all(&(result2.as_int().to_le_bytes()))?;

                match &result2 {
                    SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        // equip_error2: u32
                        w.write_all(&equip_error2.to_le_bytes())?;

                    }
                    _ => {}
                }

            }
            SMSG_SEND_MAIL_RESULT_MailAction::MoneyTaken {
                result2,
            } => {
                // result2: MailResultTwo
                w.write_all(&(result2.as_int().to_le_bytes()))?;

                match &result2 {
                    SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        // equip_error2: u32
                        w.write_all(&equip_error2.to_le_bytes())?;

                    }
                    _ => {}
                }

            }
            SMSG_SEND_MAIL_RESULT_MailAction::ItemTaken {
                result,
            } => {
                // result: MailResult
                w.write_all(&(result.as_int().to_le_bytes()))?;

                match &result {
                    SMSG_SEND_MAIL_RESULT_MailResult::Ok {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrEquipError {
                        equip_error,
                    } => {
                        // equip_error: u32
                        w.write_all(&equip_error.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrCannotSendToSelf {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrNotEnoughMoney {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrRecipientNotFound {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrNotYourTeam {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrInternalError {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrDisabledForTrialAcc {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrRecipientCapReached {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrCantSendWrappedCod {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrMailAndChatSuspended {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrTooManyAttachments {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrMailAttachmentInvalid {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                    SMSG_SEND_MAIL_RESULT_MailResult::ErrItemHasExpired {
                        item,
                        item_count,
                    } => {
                        // item: u32
                        w.write_all(&item.to_le_bytes())?;

                        // item_count: u32
                        w.write_all(&item_count.to_le_bytes())?;

                    }
                }

            }
            SMSG_SEND_MAIL_RESULT_MailAction::ReturnedToSender {
                result2,
            } => {
                // result2: MailResultTwo
                w.write_all(&(result2.as_int().to_le_bytes()))?;

                match &result2 {
                    SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        // equip_error2: u32
                        w.write_all(&equip_error2.to_le_bytes())?;

                    }
                    _ => {}
                }

            }
            SMSG_SEND_MAIL_RESULT_MailAction::Deleted {
                result2,
            } => {
                // result2: MailResultTwo
                w.write_all(&(result2.as_int().to_le_bytes()))?;

                match &result2 {
                    SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        // equip_error2: u32
                        w.write_all(&equip_error2.to_le_bytes())?;

                    }
                    _ => {}
                }

            }
            SMSG_SEND_MAIL_RESULT_MailAction::MadePermanent {
                result2,
            } => {
                // result2: MailResultTwo
                w.write_all(&(result2.as_int().to_le_bytes()))?;

                match &result2 {
                    SMSG_SEND_MAIL_RESULT_MailResultTwo::ErrEquipError {
                        equip_error2,
                    } => {
                        // equip_error2: u32
                        w.write_all(&equip_error2.to_le_bytes())?;

                    }
                    _ => {}
                }

            }
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(569, "SMSG_SEND_MAIL_RESULT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SEND_MAIL_RESULT {}

impl SMSG_SEND_MAIL_RESULT {
    pub(crate) const fn size(&self) -> usize {
        4 // mail_id: u32
        + self.action.size() // action: SMSG_SEND_MAIL_RESULT_MailAction
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_SEND_MAIL_RESULT_MailResultTwo {
    Ok,
    ErrEquipError {
        equip_error2: u32,
    },
    ErrCannotSendToSelf,
    ErrNotEnoughMoney,
    ErrRecipientNotFound,
    ErrNotYourTeam,
    ErrInternalError,
    ErrDisabledForTrialAcc,
    ErrRecipientCapReached,
    ErrCantSendWrappedCod,
    ErrMailAndChatSuspended,
    ErrTooManyAttachments,
    ErrMailAttachmentInvalid,
    ErrItemHasExpired,
}

impl Default for SMSG_SEND_MAIL_RESULT_MailResultTwo {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Ok
    }
}

impl SMSG_SEND_MAIL_RESULT_MailResultTwo {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Ok => 0,
            Self::ErrEquipError { .. } => 1,
            Self::ErrCannotSendToSelf => 2,
            Self::ErrNotEnoughMoney => 3,
            Self::ErrRecipientNotFound => 4,
            Self::ErrNotYourTeam => 5,
            Self::ErrInternalError => 6,
            Self::ErrDisabledForTrialAcc => 14,
            Self::ErrRecipientCapReached => 15,
            Self::ErrCantSendWrappedCod => 16,
            Self::ErrMailAndChatSuspended => 17,
            Self::ErrTooManyAttachments => 18,
            Self::ErrMailAttachmentInvalid => 19,
            Self::ErrItemHasExpired => 21,
        }
    }

}

impl std::fmt::Display for SMSG_SEND_MAIL_RESULT_MailResultTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::ErrEquipError{ .. } => f.write_str("ErrEquipError"),
            Self::ErrCannotSendToSelf => f.write_str("ErrCannotSendToSelf"),
            Self::ErrNotEnoughMoney => f.write_str("ErrNotEnoughMoney"),
            Self::ErrRecipientNotFound => f.write_str("ErrRecipientNotFound"),
            Self::ErrNotYourTeam => f.write_str("ErrNotYourTeam"),
            Self::ErrInternalError => f.write_str("ErrInternalError"),
            Self::ErrDisabledForTrialAcc => f.write_str("ErrDisabledForTrialAcc"),
            Self::ErrRecipientCapReached => f.write_str("ErrRecipientCapReached"),
            Self::ErrCantSendWrappedCod => f.write_str("ErrCantSendWrappedCod"),
            Self::ErrMailAndChatSuspended => f.write_str("ErrMailAndChatSuspended"),
            Self::ErrTooManyAttachments => f.write_str("ErrTooManyAttachments"),
            Self::ErrMailAttachmentInvalid => f.write_str("ErrMailAttachmentInvalid"),
            Self::ErrItemHasExpired => f.write_str("ErrItemHasExpired"),
        }
    }
}

impl SMSG_SEND_MAIL_RESULT_MailResultTwo {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::ErrEquipError {
                ..
            } => {
                4
                + 4 // equip_error2: u32
            }
            _ => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_SEND_MAIL_RESULT_MailResult {
    Ok {
        item: u32,
        item_count: u32,
    },
    ErrEquipError {
        equip_error: u32,
    },
    ErrCannotSendToSelf {
        item: u32,
        item_count: u32,
    },
    ErrNotEnoughMoney {
        item: u32,
        item_count: u32,
    },
    ErrRecipientNotFound {
        item: u32,
        item_count: u32,
    },
    ErrNotYourTeam {
        item: u32,
        item_count: u32,
    },
    ErrInternalError {
        item: u32,
        item_count: u32,
    },
    ErrDisabledForTrialAcc {
        item: u32,
        item_count: u32,
    },
    ErrRecipientCapReached {
        item: u32,
        item_count: u32,
    },
    ErrCantSendWrappedCod {
        item: u32,
        item_count: u32,
    },
    ErrMailAndChatSuspended {
        item: u32,
        item_count: u32,
    },
    ErrTooManyAttachments {
        item: u32,
        item_count: u32,
    },
    ErrMailAttachmentInvalid {
        item: u32,
        item_count: u32,
    },
    ErrItemHasExpired {
        item: u32,
        item_count: u32,
    },
}

impl Default for SMSG_SEND_MAIL_RESULT_MailResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Ok {
            item: Default::default(),
            item_count: Default::default(),
        }
    }
}

impl SMSG_SEND_MAIL_RESULT_MailResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Ok { .. } => 0,
            Self::ErrEquipError { .. } => 1,
            Self::ErrCannotSendToSelf { .. } => 2,
            Self::ErrNotEnoughMoney { .. } => 3,
            Self::ErrRecipientNotFound { .. } => 4,
            Self::ErrNotYourTeam { .. } => 5,
            Self::ErrInternalError { .. } => 6,
            Self::ErrDisabledForTrialAcc { .. } => 14,
            Self::ErrRecipientCapReached { .. } => 15,
            Self::ErrCantSendWrappedCod { .. } => 16,
            Self::ErrMailAndChatSuspended { .. } => 17,
            Self::ErrTooManyAttachments { .. } => 18,
            Self::ErrMailAttachmentInvalid { .. } => 19,
            Self::ErrItemHasExpired { .. } => 21,
        }
    }

}

impl std::fmt::Display for SMSG_SEND_MAIL_RESULT_MailResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok{ .. } => f.write_str("Ok"),
            Self::ErrEquipError{ .. } => f.write_str("ErrEquipError"),
            Self::ErrCannotSendToSelf{ .. } => f.write_str("ErrCannotSendToSelf"),
            Self::ErrNotEnoughMoney{ .. } => f.write_str("ErrNotEnoughMoney"),
            Self::ErrRecipientNotFound{ .. } => f.write_str("ErrRecipientNotFound"),
            Self::ErrNotYourTeam{ .. } => f.write_str("ErrNotYourTeam"),
            Self::ErrInternalError{ .. } => f.write_str("ErrInternalError"),
            Self::ErrDisabledForTrialAcc{ .. } => f.write_str("ErrDisabledForTrialAcc"),
            Self::ErrRecipientCapReached{ .. } => f.write_str("ErrRecipientCapReached"),
            Self::ErrCantSendWrappedCod{ .. } => f.write_str("ErrCantSendWrappedCod"),
            Self::ErrMailAndChatSuspended{ .. } => f.write_str("ErrMailAndChatSuspended"),
            Self::ErrTooManyAttachments{ .. } => f.write_str("ErrTooManyAttachments"),
            Self::ErrMailAttachmentInvalid{ .. } => f.write_str("ErrMailAttachmentInvalid"),
            Self::ErrItemHasExpired{ .. } => f.write_str("ErrItemHasExpired"),
        }
    }
}

impl SMSG_SEND_MAIL_RESULT_MailResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Ok {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrEquipError {
                ..
            } => {
                4
                + 4 // equip_error: u32
            }
            Self::ErrCannotSendToSelf {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrNotEnoughMoney {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrRecipientNotFound {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrNotYourTeam {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrInternalError {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrDisabledForTrialAcc {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrRecipientCapReached {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrCantSendWrappedCod {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrMailAndChatSuspended {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrTooManyAttachments {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrMailAttachmentInvalid {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
            Self::ErrItemHasExpired {
                ..
            } => {
                4
                + 4 // item: u32
                + 4 // item_count: u32
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_SEND_MAIL_RESULT_MailAction {
    Send {
        result2: SMSG_SEND_MAIL_RESULT_MailResultTwo,
    },
    MoneyTaken {
        result2: SMSG_SEND_MAIL_RESULT_MailResultTwo,
    },
    ItemTaken {
        result: SMSG_SEND_MAIL_RESULT_MailResult,
    },
    ReturnedToSender {
        result2: SMSG_SEND_MAIL_RESULT_MailResultTwo,
    },
    Deleted {
        result2: SMSG_SEND_MAIL_RESULT_MailResultTwo,
    },
    MadePermanent {
        result2: SMSG_SEND_MAIL_RESULT_MailResultTwo,
    },
}

impl Default for SMSG_SEND_MAIL_RESULT_MailAction {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Send {
            result2: Default::default(),
        }
    }
}

impl SMSG_SEND_MAIL_RESULT_MailAction {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Send { .. } => 0,
            Self::MoneyTaken { .. } => 1,
            Self::ItemTaken { .. } => 2,
            Self::ReturnedToSender { .. } => 3,
            Self::Deleted { .. } => 4,
            Self::MadePermanent { .. } => 5,
        }
    }

}

impl std::fmt::Display for SMSG_SEND_MAIL_RESULT_MailAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Send{ .. } => f.write_str("Send"),
            Self::MoneyTaken{ .. } => f.write_str("MoneyTaken"),
            Self::ItemTaken{ .. } => f.write_str("ItemTaken"),
            Self::ReturnedToSender{ .. } => f.write_str("ReturnedToSender"),
            Self::Deleted{ .. } => f.write_str("Deleted"),
            Self::MadePermanent{ .. } => f.write_str("MadePermanent"),
        }
    }
}

impl SMSG_SEND_MAIL_RESULT_MailAction {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Send {
                result2,
            } => {
                4
                + result2.size() // result2: SMSG_SEND_MAIL_RESULT_MailResultTwo
            }
            Self::MoneyTaken {
                result2,
            } => {
                4
                + result2.size() // result2: SMSG_SEND_MAIL_RESULT_MailResultTwo
            }
            Self::ItemTaken {
                result,
            } => {
                4
                + result.size() // result: SMSG_SEND_MAIL_RESULT_MailResult
            }
            Self::ReturnedToSender {
                result2,
            } => {
                4
                + result2.size() // result2: SMSG_SEND_MAIL_RESULT_MailResultTwo
            }
            Self::Deleted {
                result2,
            } => {
                4
                + result2.size() // result2: SMSG_SEND_MAIL_RESULT_MailResultTwo
            }
            Self::MadePermanent {
                result2,
            } => {
                4
                + result2.size() // result2: SMSG_SEND_MAIL_RESULT_MailResultTwo
            }
        }
    }
}

