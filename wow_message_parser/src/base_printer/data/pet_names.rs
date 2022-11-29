use rusqlite::Connection;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum Pet {
    Imp,
    Felhunter,
    Voidwalker,
    Succubus,
    Felguard,
    RisenGhoul,
}

impl Pet {
    pub fn from_int(v: u16) -> Self {
        match v {
            416 => Pet::Imp,
            417 => Pet::Felhunter,
            1860 => Pet::Voidwalker,
            1863 => Pet::Succubus,
            17252 => Pet::Felguard,
            26125 => Pet::RisenGhoul,
            v => panic!("Invalid warlock pet '{}'", v),
        }
    }
}

pub(crate) struct PetNames {
    pub first_names: Vec<String>,
    pub last_names: Vec<String>,
}

pub(crate) fn get_pet_name_data(conn: &Connection) -> BTreeMap<Pet, PetNames> {
    let mut c = conn
        .prepare("SELECT word, entry, half FROM pet_name_generation;")
        .unwrap();

    let mut map: BTreeMap<Pet, PetNames> = BTreeMap::new();

    let mut rows = c.query([]).unwrap();
    while let Ok(Some(row)) = rows.next() {
        let entry: u16 = row.get(1).unwrap();
        let word: String = row.get(0).unwrap();
        let half: u8 = row.get(2).unwrap();

        let pet = Pet::from_int(entry);

        if let Some(names) = map.get_mut(&pet) {
            if half == 0 {
                names.first_names.push(word);
            } else {
                names.last_names.push(word);
            }
        } else {
            let mut last_names = Vec::new();
            let mut first_names = Vec::new();
            if half == 0 {
                first_names.push(word);
            } else {
                last_names.push(word);
            }
            map.insert(
                pet,
                PetNames {
                    first_names,
                    last_names,
                },
            );
        }
    }

    map
}
