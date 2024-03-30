use std::io::{Read, Write};

use crate::wrath::{
    Area, Map, WorldState,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_init_world_states.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_init_world_states.wowm#L10):
/// ```text
/// smsg SMSG_INIT_WORLD_STATES = 0x02C2 {
///     Map map;
///     Area area;
///     Area sub_area;
///     u16 amount_of_states;
///     WorldState[amount_of_states] states;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_INIT_WORLD_STATES {
    pub map: Map,
    pub area: Area,
    pub sub_area: Area,
    pub states: Vec<WorldState>,
}

impl crate::private::Sealed for SMSG_INIT_WORLD_STATES {}
impl SMSG_INIT_WORLD_STATES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=524302).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // sub_area: Area
        let sub_area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // amount_of_states: u16
        let amount_of_states = crate::util::read_u16_le(&mut r)?;

        // states: WorldState[amount_of_states]
        let states = {
            let mut states = Vec::with_capacity(amount_of_states as usize);
            for _ in 0..amount_of_states {
                states.push(WorldState::read(&mut r)?);
            }
            states
        };

        Ok(Self {
            map,
            area,
            sub_area,
            states,
        })
    }

}

impl crate::Message for SMSG_INIT_WORLD_STATES {
    const OPCODE: u32 = 0x02c2;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_INIT_WORLD_STATES"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INIT_WORLD_STATES {{").unwrap();
        // Members
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    area = {};", self.area.as_test_case_value()).unwrap();
        writeln!(s, "    sub_area = {};", self.sub_area.as_test_case_value()).unwrap();
        writeln!(s, "    amount_of_states = {};", self.states.len()).unwrap();
        writeln!(s, "    states = [").unwrap();
        for v in self.states.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            state = {};", v.state).unwrap();
            writeln!(s, "            value = {};", v.value).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 706_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "sub_area", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "amount_of_states", "    ");
        if !self.states.is_empty() {
            writeln!(s, "    /* states: WorldState[amount_of_states] start */").unwrap();
            for (i, v) in self.states.iter().enumerate() {
                writeln!(s, "    /* states: WorldState[amount_of_states] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "state", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "value", "        ");
                writeln!(s, "    /* states: WorldState[amount_of_states] {i} end */").unwrap();
            }
            writeln!(s, "    /* states: WorldState[amount_of_states] end */").unwrap();
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
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // sub_area: Area
        w.write_all(&(self.sub_area.as_int().to_le_bytes()))?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes())?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(706, "SMSG_INIT_WORLD_STATES", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INIT_WORLD_STATES {}

impl SMSG_INIT_WORLD_STATES {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + 4 // area: Area
        + 4 // sub_area: Area
        + 2 // amount_of_states: u16
        + self.states.len() * 8 // states: WorldState[amount_of_states]
    }
}

