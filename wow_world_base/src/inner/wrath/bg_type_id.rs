/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm:64`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm#L64):
/// ```text
/// enum BgTypeId : u32 {
///     NOT_ELIGIBLE = 0;
///     QUEUED_FOR_AV = 1;
///     QUEUED_FOR_WSG = 2;
///     QUEUED_FOR_AB = 3;
///     QUEUED_FOR_NETHERSTORM = 4;
///     QUEUED_FOR_BLADES_EDGE_ARENA = 5;
///     QUEUED_FOR_ARENA = 6;
///     QUEUED_FOR_EYE_OF_THE_STORM = 7;
///     QUEUED_FOR_RUINS_OF_LORDAERON = 8;
///     QUEUED_FOR_STRAND_OF_THE_ANCIENT = 9;
///     QUEUED_FOR_DALARAN_SEWERS = 10;
///     QUEUED_FOR_RING_OF_VALOR = 11;
///     QUEUED_FOR_ISLE_OF_CONQUEST = 30;
///     REMOVE_FROM_QUEUE = 0xFFFFFFFE;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BgTypeId {
    /// Your group has joined a battleground queue, but you are not eligible
    NotEligible,
    /// Your group has joined the queue for AV
    QueuedForAv,
    /// Your group has joined the queue for WS
    QueuedForWsg,
    /// Your group has joined the queue for AB
    QueuedForAb,
    QueuedForNetherstorm,
    QueuedForBladesEdgeArena,
    QueuedForArena,
    QueuedForEyeOfTheStorm,
    QueuedForRuinsOfLordaeron,
    QueuedForStrandOfTheAncient,
    QueuedForDalaranSewers,
    QueuedForRingOfValor,
    QueuedForIsleOfConquest,
    /// send bg command result to show nice message
    RemoveFromQueue,
}

impl BgTypeId {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::NotEligible => 0x0,
            Self::QueuedForAv => 0x1,
            Self::QueuedForWsg => 0x2,
            Self::QueuedForAb => 0x3,
            Self::QueuedForNetherstorm => 0x4,
            Self::QueuedForBladesEdgeArena => 0x5,
            Self::QueuedForArena => 0x6,
            Self::QueuedForEyeOfTheStorm => 0x7,
            Self::QueuedForRuinsOfLordaeron => 0x8,
            Self::QueuedForStrandOfTheAncient => 0x9,
            Self::QueuedForDalaranSewers => 0xa,
            Self::QueuedForRingOfValor => 0xb,
            Self::QueuedForIsleOfConquest => 0x1e,
            Self::RemoveFromQueue => 0xfffffffe,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl BgTypeId {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NotEligible => "NOT_ELIGIBLE",
            Self::QueuedForAv => "QUEUED_FOR_AV",
            Self::QueuedForWsg => "QUEUED_FOR_WSG",
            Self::QueuedForAb => "QUEUED_FOR_AB",
            Self::QueuedForNetherstorm => "QUEUED_FOR_NETHERSTORM",
            Self::QueuedForBladesEdgeArena => "QUEUED_FOR_BLADES_EDGE_ARENA",
            Self::QueuedForArena => "QUEUED_FOR_ARENA",
            Self::QueuedForEyeOfTheStorm => "QUEUED_FOR_EYE_OF_THE_STORM",
            Self::QueuedForRuinsOfLordaeron => "QUEUED_FOR_RUINS_OF_LORDAERON",
            Self::QueuedForStrandOfTheAncient => "QUEUED_FOR_STRAND_OF_THE_ANCIENT",
            Self::QueuedForDalaranSewers => "QUEUED_FOR_DALARAN_SEWERS",
            Self::QueuedForRingOfValor => "QUEUED_FOR_RING_OF_VALOR",
            Self::QueuedForIsleOfConquest => "QUEUED_FOR_ISLE_OF_CONQUEST",
            Self::RemoveFromQueue => "REMOVE_FROM_QUEUE",
        }
    }

}

impl Default for BgTypeId {
    fn default() -> Self {
        Self::NotEligible
    }
}

impl std::fmt::Display for BgTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEligible => f.write_str("NotEligible"),
            Self::QueuedForAv => f.write_str("QueuedForAv"),
            Self::QueuedForWsg => f.write_str("QueuedForWsg"),
            Self::QueuedForAb => f.write_str("QueuedForAb"),
            Self::QueuedForNetherstorm => f.write_str("QueuedForNetherstorm"),
            Self::QueuedForBladesEdgeArena => f.write_str("QueuedForBladesEdgeArena"),
            Self::QueuedForArena => f.write_str("QueuedForArena"),
            Self::QueuedForEyeOfTheStorm => f.write_str("QueuedForEyeOfTheStorm"),
            Self::QueuedForRuinsOfLordaeron => f.write_str("QueuedForRuinsOfLordaeron"),
            Self::QueuedForStrandOfTheAncient => f.write_str("QueuedForStrandOfTheAncient"),
            Self::QueuedForDalaranSewers => f.write_str("QueuedForDalaranSewers"),
            Self::QueuedForRingOfValor => f.write_str("QueuedForRingOfValor"),
            Self::QueuedForIsleOfConquest => f.write_str("QueuedForIsleOfConquest"),
            Self::RemoveFromQueue => f.write_str("RemoveFromQueue"),
        }
    }
}

impl TryFrom<u32> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotEligible),
            1 => Ok(Self::QueuedForAv),
            2 => Ok(Self::QueuedForWsg),
            3 => Ok(Self::QueuedForAb),
            4 => Ok(Self::QueuedForNetherstorm),
            5 => Ok(Self::QueuedForBladesEdgeArena),
            6 => Ok(Self::QueuedForArena),
            7 => Ok(Self::QueuedForEyeOfTheStorm),
            8 => Ok(Self::QueuedForRuinsOfLordaeron),
            9 => Ok(Self::QueuedForStrandOfTheAncient),
            10 => Ok(Self::QueuedForDalaranSewers),
            11 => Ok(Self::QueuedForRingOfValor),
            30 => Ok(Self::QueuedForIsleOfConquest),
            4294967294 => Ok(Self::RemoveFromQueue),
            v => Err(crate::errors::EnumError::new("BgTypeId", v.into()),)
        }
    }
}

