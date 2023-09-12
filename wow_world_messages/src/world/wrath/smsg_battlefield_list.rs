use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    BattlegroundType, RandomBg,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:83`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L83):
/// ```text
/// smsg SMSG_BATTLEFIELD_LIST = 0x023D {
///     Guid battlemaster;
///     BattlegroundType battleground_type;
///     u8 unknown1;
///     u8 unknown2;
///     u8 has_win;
///     u32 win_honor;
///     u32 win_arena;
///     u32 loss_honor;
///     RandomBg random;
///     if (random == RANDOM) {
///         u8 win_random;
///         u32 reward_honor;
///         u32 reward_arena;
///         u32 honor_lost;
///     }
///     u32 number_of_battlegrounds;
///     u32[number_of_battlegrounds] battlegrounds;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_LIST {
    pub battlemaster: Guid,
    pub battleground_type: BattlegroundType,
    pub unknown1: u8,
    pub unknown2: u8,
    pub has_win: u8,
    pub win_honor: u32,
    pub win_arena: u32,
    pub loss_honor: u32,
    pub random: SMSG_BATTLEFIELD_LIST_RandomBg,
    pub battlegrounds: Vec<u32>,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_LIST {}
impl SMSG_BATTLEFIELD_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(32..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // battlemaster: Guid
        let battlemaster = crate::util::read_guid(&mut r)?;

        // battleground_type: BattlegroundType
        let battleground_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(&mut r)?;

        // has_win: u8
        let has_win = crate::util::read_u8_le(&mut r)?;

        // win_honor: u32
        let win_honor = crate::util::read_u32_le(&mut r)?;

        // win_arena: u32
        let win_arena = crate::util::read_u32_le(&mut r)?;

        // loss_honor: u32
        let loss_honor = crate::util::read_u32_le(&mut r)?;

        // random: RandomBg
        let random = crate::util::read_u8_le(&mut r)?.try_into()?;

        let random_if = match random {
            RandomBg::NotRandom => SMSG_BATTLEFIELD_LIST_RandomBg::NotRandom,
            RandomBg::Random => {
                // win_random: u8
                let win_random = crate::util::read_u8_le(&mut r)?;

                // reward_honor: u32
                let reward_honor = crate::util::read_u32_le(&mut r)?;

                // reward_arena: u32
                let reward_arena = crate::util::read_u32_le(&mut r)?;

                // honor_lost: u32
                let honor_lost = crate::util::read_u32_le(&mut r)?;

                SMSG_BATTLEFIELD_LIST_RandomBg::Random {
                    honor_lost,
                    reward_arena,
                    reward_honor,
                    win_random,
                }
            }
        };

        // number_of_battlegrounds: u32
        let number_of_battlegrounds = crate::util::read_u32_le(&mut r)?;

        // battlegrounds: u32[number_of_battlegrounds]
        let battlegrounds = {
            let mut battlegrounds = Vec::with_capacity(number_of_battlegrounds as usize);
            for _ in 0..number_of_battlegrounds {
                battlegrounds.push(crate::util::read_u32_le(&mut r)?);
            }
            battlegrounds
        };

        Ok(Self {
            battlemaster,
            battleground_type,
            unknown1,
            unknown2,
            has_win,
            win_honor,
            win_arena,
            loss_honor,
            random: random_if,
            battlegrounds,
        })
    }

}

impl crate::Message for SMSG_BATTLEFIELD_LIST {
    const OPCODE: u32 = 0x023d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_BATTLEFIELD_LIST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_LIST {{").unwrap();
        // Members
        writeln!(s, "    battlemaster = {};", self.battlemaster.guid()).unwrap();
        writeln!(s, "    battleground_type = {};", self.battleground_type.as_test_case_value()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        writeln!(s, "    has_win = {};", self.has_win).unwrap();
        writeln!(s, "    win_honor = {};", self.win_honor).unwrap();
        writeln!(s, "    win_arena = {};", self.win_arena).unwrap();
        writeln!(s, "    loss_honor = {};", self.loss_honor).unwrap();
        writeln!(s, "    random = {};", RandomBg::try_from(self.random.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.random {
            crate::wrath::SMSG_BATTLEFIELD_LIST_RandomBg::Random {
                honor_lost,
                reward_arena,
                reward_honor,
                win_random,
            } => {
                writeln!(s, "    win_random = {};", win_random).unwrap();
                writeln!(s, "    reward_honor = {};", reward_honor).unwrap();
                writeln!(s, "    reward_arena = {};", reward_arena).unwrap();
                writeln!(s, "    honor_lost = {};", honor_lost).unwrap();
            }
            _ => {}
        }

        writeln!(s, "    number_of_battlegrounds = {};", self.battlegrounds.len()).unwrap();
        write!(s, "    battlegrounds = [").unwrap();
        for v in self.battlegrounds.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 573_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "battlemaster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "battleground_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "has_win", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "win_honor", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "win_arena", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "loss_honor", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "random", "    ");
        match &self.random {
            crate::wrath::SMSG_BATTLEFIELD_LIST_RandomBg::Random {
                honor_lost,
                reward_arena,
                reward_honor,
                win_random,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "win_random", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_honor", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_arena", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_lost", "    ");
            }
            _ => {}
        }

        crate::util::write_bytes(&mut s, &mut bytes, 4, "number_of_battlegrounds", "    ");
        if !self.battlegrounds.is_empty() {
            writeln!(s, "    /* battlegrounds: u32[number_of_battlegrounds] start */").unwrap();
            for (i, v) in self.battlegrounds.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("battlegrounds {i}"), "    ");
            }
            writeln!(s, "    /* battlegrounds: u32[number_of_battlegrounds] end */").unwrap();
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
        // battlemaster: Guid
        w.write_all(&self.battlemaster.guid().to_le_bytes())?;

        // battleground_type: BattlegroundType
        w.write_all(&(self.battleground_type.as_int().to_le_bytes()))?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // has_win: u8
        w.write_all(&self.has_win.to_le_bytes())?;

        // win_honor: u32
        w.write_all(&self.win_honor.to_le_bytes())?;

        // win_arena: u32
        w.write_all(&self.win_arena.to_le_bytes())?;

        // loss_honor: u32
        w.write_all(&self.loss_honor.to_le_bytes())?;

        // random: RandomBg
        w.write_all(&(self.random.as_int().to_le_bytes()))?;

        match &self.random {
            SMSG_BATTLEFIELD_LIST_RandomBg::Random {
                honor_lost,
                reward_arena,
                reward_honor,
                win_random,
            } => {
                // win_random: u8
                w.write_all(&win_random.to_le_bytes())?;

                // reward_honor: u32
                w.write_all(&reward_honor.to_le_bytes())?;

                // reward_arena: u32
                w.write_all(&reward_arena.to_le_bytes())?;

                // honor_lost: u32
                w.write_all(&honor_lost.to_le_bytes())?;

            }
            _ => {}
        }

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes())?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(573, "SMSG_BATTLEFIELD_LIST", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_LIST {}

impl SMSG_BATTLEFIELD_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // battlemaster: Guid
        + 4 // battleground_type: BattlegroundType
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + 1 // has_win: u8
        + 4 // win_honor: u32
        + 4 // win_arena: u32
        + 4 // loss_honor: u32
        + self.random.size() // random: SMSG_BATTLEFIELD_LIST_RandomBg
        + 4 // number_of_battlegrounds: u32
        + self.battlegrounds.len() * core::mem::size_of::<u32>() // battlegrounds: u32[number_of_battlegrounds]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_BATTLEFIELD_LIST_RandomBg {
    NotRandom,
    Random {
        honor_lost: u32,
        reward_arena: u32,
        reward_honor: u32,
        win_random: u8,
    },
}

impl Default for SMSG_BATTLEFIELD_LIST_RandomBg {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotRandom
    }
}

impl SMSG_BATTLEFIELD_LIST_RandomBg {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotRandom => 0,
            Self::Random { .. } => 1,
        }
    }

}

impl std::fmt::Display for SMSG_BATTLEFIELD_LIST_RandomBg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotRandom => f.write_str("NotRandom"),
            Self::Random{ .. } => f.write_str("Random"),
        }
    }
}

impl SMSG_BATTLEFIELD_LIST_RandomBg {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Random {
                ..
            } => {
                1
                + 4 // honor_lost: u32
                + 4 // reward_arena: u32
                + 4 // reward_honor: u32
                + 1 // win_random: u8
            }
            _ => 1,
        }
    }
}

