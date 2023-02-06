mod constructor;
pub(crate) mod conversions;
pub(crate) mod definition;

use crate::base_printer::data::items::{Array, Field, IntegerSize, Optimizations, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub(crate) struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub stamina: i32,
    pub intellect: i32,
    pub spirit: i32,
    pub health: i32,
    pub mana: i32,
}

use crate::base_printer::write::items::constructor::constructor;
use crate::base_printer::write::items::definition::{definition, includes};
use crate::base_printer::writer::Writer;
use crate::base_printer::{Expansion, ImportFrom};
use crate::file_utils::overwrite_autogenerate_if_not_the_same;
use std::path::Path;

pub struct GenericThing {
    pub entry: u32,
    pub extra_flags: i32,
    pub name: String,
    pub fields: Vec<Field>,
    pub arrays: Vec<Array>,
}

impl GenericThing {
    pub fn types_that_are_defaulted(&self) -> BTreeSet<&'static str> {
        let mut types_that_are_defaulted = BTreeSet::new();

        for array in &self.arrays {
            if array.is_default() {
                types_that_are_defaulted.insert(array.type_name);
            }
        }

        types_that_are_defaulted
    }

    fn ty_to_short(n: &str) -> &'static str {
        match n {
            "ItemDamageType" => "a",
            "Spells" => "b",
            "ItemSocket" => "c",
            "ItemStat" => "d",
            "Reagent" => "e",
            "SpellEffect" => "f",
            "Totem" => "g",
            "TotemCategory" => "h",
            v => unimplemented!("Unhandled array type {}", v),
        }
    }

    pub fn constructor_name(&self) -> String {
        let mut s = "n".to_string();

        for n in self.types_that_are_defaulted() {
            let n = Self::ty_to_short(n);
            s.push_str(n);
        }

        s
    }

    pub fn new(
        entry: u32,
        extra_flags: i32,
        name: String,
        fields: Vec<Field>,
        arrays: Vec<Array>,
    ) -> Self {
        Self {
            entry,
            extra_flags,
            name,
            fields,
            arrays,
        }
    }
}

pub(crate) fn write_definition(
    path: &Path,
    fields: &[Field],
    arrays: &[Array],
    expansion: Expansion,
    ty_name: &str,
    optimizations: &Optimizations,
) {
    let mut s = Writer::new();

    definition(&mut s, fields, arrays, expansion, ty_name, optimizations);

    overwrite_autogenerate_if_not_the_same(path, s.inner());
}

pub(crate) fn write_pub_use(
    path: &Path,
    things: &[GenericThing],
    expansion: Expansion,
    ty_name: &str,
    optimizations: &Optimizations,
) {
    let mut s = Writer::new();
    s.w("pub ");

    includes(
        &mut s,
        &things[0].fields,
        &things[0].arrays,
        expansion,
        ImportFrom::ItemPubUse,
        ty_name,
        optimizations,
    );

    overwrite_autogenerate_if_not_the_same(path, s.inner());
}

pub(crate) fn write_constructors(
    path: &Path,
    things: &[GenericThing],
    expansion: Expansion,
    ty_name: &str,
    optimizations: &Optimizations,
) {
    let mut s = Writer::new();

    constructor(&mut s, things, expansion, ty_name, optimizations);

    overwrite_autogenerate_if_not_the_same(path, s.inner());
}

pub(crate) fn write_things(
    path: &Path,
    things: &[GenericThing],
    expansion: Expansion,
    ty_name: &str,
    unobtainable: impl Fn(&GenericThing) -> bool,
    optimizations: &Optimizations,
) {
    let mut s = Writer::new();

    let default_values = get_default_values(things, optimizations);
    const_default_values(&mut s, &default_values);

    all_items(
        &mut s,
        things,
        expansion,
        unobtainable,
        ty_name,
        &default_values,
        optimizations,
    );

    overwrite_autogenerate_if_not_the_same(path, s.inner());
}

fn get_default_values(
    things: &[GenericThing],
    optimizations: &Optimizations,
) -> BTreeMap<(Value, Option<IntegerSize>), String> {
    let mut map: HashMap<&'static str, BTreeMap<(Value, Option<IntegerSize>), usize>> =
        HashMap::new();

    for thing in things {
        for field in &thing.fields {
            if optimizations.optimization(field.name).skip_field() {
                continue;
            }

            insert_value(&mut map, &field.value, optimizations.integer_size(field))
        }

        for array in &thing.arrays {
            for instance in &array.instances {
                for field in instance {
                    insert_value(&mut map, &field.value, field.integer_size())
                }
            }
        }
    }

    let mut set = BTreeMap::new();

    for (_, values) in map {
        let mut v = values.iter().next().unwrap();
        for value in &values {
            if value.1 > v.1 {
                v = value;
            }
        }

        set.insert(v.0.clone(), v.0 .0.const_name().to_string());
    }

    set
}

fn insert_value(
    map: &mut HashMap<&str, BTreeMap<(Value, Option<IntegerSize>), usize>>,
    value: &Value,
    integer_size: Option<IntegerSize>,
) {
    if let Some(set) = map.get_mut(value.const_name()) {
        if let Some(v) = set.get_mut(&(value.clone(), integer_size)) {
            *v += 1;
        } else {
            set.insert((value.clone(), integer_size), 1);
        }
    } else {
        let mut set = BTreeMap::new();

        set.insert((value.clone(), integer_size), 1);

        map.insert(value.const_name(), set);
    }
}

fn const_default_values(
    s: &mut Writer,
    default_values: &BTreeMap<(Value, Option<IntegerSize>), String>,
) {
    for ((value, integer_size), const_name) in default_values {
        let ty_name = if let Some(t) = integer_size {
            t.string_value()
        } else {
            value.constructor_type_name()
        };

        s.wln(format!(
            "const {const_name}: {ty_name} = {};",
            value.to_string_value(),
        ));
    }
}

pub(crate) fn unobtainable_item(entry: u32, extra_flags: i32, name: &str) -> bool {
    const UNOBTAINABLE_FLAG: i32 = 0x04;
    let unobtainable_flag_is_set = extra_flags & UNOBTAINABLE_FLAG != 0;

    let name_ends_with_deprecated = name.ends_with("DEPRECATED") || name.ends_with("DEP");
    let name_ends_with_test = name.ends_with(" Test") || name.ends_with("(Test)");

    let name_starts_with_old = name.starts_with("OLD") || name.starts_with("(OLD)");
    let name_starts_with_monster = name.starts_with("Monster - ");
    let name_starts_with_test = name.starts_with("TEST ");
    let name_starts_with_deprecated = name.starts_with("Deprecated");

    let name_contains_ph = name.contains("[PH]");

    let martin_thunder_or_martin_fury = entry == 17 || entry == 192;

    let glaive_of_the_defender = entry == 23051;

    let warglaives_of_azzinoth = entry == 18582 || entry == 18583 || entry == 18584;

    unobtainable_flag_is_set
        || name_ends_with_deprecated
        || name_starts_with_old
        || name_ends_with_test
        || name_starts_with_monster
        || name_starts_with_test
        || name_starts_with_deprecated
        || name_contains_ph
        || martin_thunder_or_martin_fury
        || glaive_of_the_defender
        || warglaives_of_azzinoth
}

fn print_unobtainable_cfg(s: &mut Writer) {
    s.wln("#[cfg(feature = \"unobtainable-items\")]");
}

fn all_items(
    s: &mut Writer,
    items: &[GenericThing],
    expansion: Expansion,
    unobtainable: impl Fn(&GenericThing) -> bool,
    ty_name: &str,
    default_values: &BTreeMap<(Value, Option<IntegerSize>), String>,
    optimizations: &Optimizations,
) {
    includes(
        s,
        &items[0].fields,
        &items[0].arrays,
        expansion,
        ImportFrom::Items,
        ty_name,
        optimizations,
    );

    s.wln(format!("pub const DATA: &[{ty_name}] = &["));

    for item in items {
        if unobtainable(item) {
            print_unobtainable_cfg(s);
        }
        s.w(format!("{}(", item.constructor_name()));

        for value in &item.fields {
            if optimizations.optimization(value.name).skip_field() {
                continue;
            }

            if let Some(const_name) =
                default_values.get(&(value.value.clone(), optimizations.integer_size(value)))
            {
                s.w_no_indent(format!("{const_name},"));
            } else {
                s.w_no_indent(format!("{},", value.value.to_string_value()));
            }
        }

        for array in &item.arrays {
            if array.is_default() {
                continue;
            }

            for instance in &array.instances {
                for field in instance {
                    if let Some(const_name) =
                        default_values.get(&(field.value.clone(), field.integer_size()))
                    {
                        s.w_no_indent(format!("{const_name},"));
                    } else {
                        s.w_no_indent(format!("{},", field.value.to_string_value()));
                    }
                }
            }
        }

        s.wln_no_indent("),");
    }

    s.wln("];");
}
