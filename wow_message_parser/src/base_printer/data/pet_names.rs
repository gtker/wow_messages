use crate::base_printer::read_csv_file;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

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
            _ => panic!("Invalid warlock pet '{v}'"),
        }
    }
}

pub(crate) struct PetNames {
    pub first_names: Vec<String>,
    pub last_names: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct PetNameGeneration {
    pub word: String,
    pub entry: u16,
    pub half: u8,
}

pub(crate) fn get_pet_name_data(dir: &Path) -> BTreeMap<Pet, PetNames> {
    let mut map: BTreeMap<Pet, PetNames> = BTreeMap::new();
    for row in read_csv_file::<PetNameGeneration>(&dir, "pet_names") {
        let entry = row.entry;
        let word: String = row.word;
        let half: u8 = row.half;

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
