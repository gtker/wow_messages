pub(crate) mod all_items;
pub(crate) mod constructor;
pub(crate) mod conversions;
pub(crate) mod definition;

use crate::base_printer::data::items::{
    Array, ArrayInstances, Field, IntegerSize, Optimizations, Value,
};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub stamina: i32,
    pub intellect: i32,
    pub spirit: i32,
    pub health: i32,
    pub mana: i32,
}

use crate::base_printer::write::items::all_items::all_items;
use crate::base_printer::write::items::constructor::constructor;
use crate::base_printer::write::items::definition::{definition, includes};
use crate::base_printer::{Expansion, ImportFrom};
use crate::file_utils::overwrite_autogenerate_if_not_the_same;
use crate::rust_printer::Writer;
use hashbrown::HashMap;
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
            "SpellEffects" => "f",
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
    lib_functions(&mut s, ty_name);

    overwrite_autogenerate_if_not_the_same(path, s.inner());
}

fn lib_functions(s: &mut Writer, ty_name: &str) {
    let ty_lower = ty_name.to_lowercase();
    s.open_curly(format!(
        "pub fn lookup_{ty_lower}(id: u32) -> Option<&'static {ty_name}>"
    ));
    s.wln(format!(
        "all_{ty_lower}s().iter().find(|a| a.entry() == id)"
    ));
    s.closing_curly();
    s.newline();

    s.open_curly(format!(
        "pub const fn all_{ty_lower}s() -> &'static [{ty_name}]"
    ));
    s.wln("data::DATA");
    s.closing_curly();
    s.newline();
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

    let (default_values, arrays) = get_default_values(things, optimizations);
    const_default_values(&mut s, &default_values);
    print_arrays(&mut s, &arrays);

    all_items(
        &mut s,
        things,
        expansion,
        unobtainable,
        ty_name,
        &default_values,
        &arrays,
        optimizations,
    );

    overwrite_autogenerate_if_not_the_same(path, s.inner());
}

struct ConstNamer {
    current: String,
    alphabet_index: usize,
}

impl ConstNamer {
    const ALPHABET: [char; 62] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '_',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    const MAX_LETTER_INDEX: usize = 25; // Only letters can start identifier

    pub fn new() -> Self {
        Self {
            current: "".to_string(),
            alphabet_index: 0,
        }
    }

    fn increment_index(&mut self) {
        self.alphabet_index += 1;

        if (self.current.is_empty() && self.alphabet_index >= Self::MAX_LETTER_INDEX)
            || self.alphabet_index >= Self::ALPHABET.len()
        {
            self.alphabet_index = 0;
            self.current.push(Self::ALPHABET[self.alphabet_index]);
        }
    }

    pub fn next(&mut self) -> String {
        let s = self.try_next();
        self.increment_index();
        s
    }

    pub fn try_next(&self) -> String {
        let mut s = self.current.clone();
        s.push(Self::ALPHABET[self.alphabet_index]);
        s
    }
}

type Values = HashMap<(Value, Option<IntegerSize>), String>;
type Arrays<'a> = HashMap<(&'a ArrayInstances, &'static str), String>;

fn get_default_values<'a>(
    things: &'a [GenericThing],
    optimizations: &Optimizations,
) -> (Values, Arrays<'a>) {
    let mut values: BTreeMap<(Value, Option<IntegerSize>), usize> = BTreeMap::new();
    for thing in things {
        for (field_index, field) in thing.fields.iter().enumerate() {
            if optimizations.optimization(field_index).skip_field() {
                continue;
            }

            let value = field.value.const_value();
            let value_and_size = (value, optimizations.integer_size(field_index));

            if let Some(g) = values.get_mut(&value_and_size) {
                *g += 1;
            } else {
                values.insert(value_and_size, 1);
            }
        }
    }

    let mut arrays = BTreeMap::new();
    for thing in things {
        for array in &thing.arrays {
            if array.is_default() {
                continue;
            }

            if let Some(amount) = arrays.get_mut(&(&array.instances, array.type_name)) {
                *amount += 1;
            } else {
                arrays.insert((&array.instances, array.type_name), 1);
            }
        }
    }

    enum ValuesWrapper<'a> {
        Values(((Value, Option<IntegerSize>), usize)),
        Arrays(((&'a ArrayInstances, &'static str), usize)),
    }

    // Ensure that most used consts have the shortest name
    let values = values.into_iter().map(ValuesWrapper::Values);
    let arrays = arrays.into_iter().map(ValuesWrapper::Arrays);

    let mut values: Vec<_> = values.chain(arrays).collect();
    values.sort_by(|a, b| match a {
        ValuesWrapper::Values((_, amount)) => match b {
            ValuesWrapper::Values((_, amount2)) => amount2.cmp(amount),
            ValuesWrapper::Arrays((_, amount2)) => amount2.cmp(amount),
        },
        ValuesWrapper::Arrays((_, amount)) => match b {
            ValuesWrapper::Values((_, amount2)) => amount2.cmp(amount),
            ValuesWrapper::Arrays((_, amount2)) => amount2.cmp(amount),
        },
    });

    let mut namer = ConstNamer::new();
    let mut values_output = HashMap::new();
    let mut arrays_output = HashMap::new();
    for value in values {
        match value {
            ValuesWrapper::Values((value, amount)) => {
                let ty_name = if let Some(t) = value.1 {
                    t.string_value()
                } else {
                    value.0.const_variable_type_name()
                };
                let try_name = namer.try_next();
                let definition =
                    get_const_definition(&try_name, ty_name, &value.0.to_string_value());

                let definition_characters = definition.len() + amount * try_name.len();
                let no_definition_characters = amount * value.0.to_string_value().len();

                if definition_characters <= no_definition_characters {
                    values_output.insert(value, namer.next());
                }
            }
            ValuesWrapper::Arrays((array, _)) => {
                arrays_output.insert(array, namer.next());
            }
        }
    }

    (values_output, arrays_output)
}

fn print_arrays(s: &mut Writer, arrays: &Arrays) {
    for ((array, ty_name), const_name) in arrays {
        s.w(format!("const {const_name}:&[{ty_name}]=&["));

        for instance in array.instances() {
            s.w_no_indent(format!("{ty_name}{{"));

            for field in instance.fields() {
                s.w_no_indent(format!(
                    "{}: {},",
                    field.name,
                    field.value.to_string_value()
                ))
            }

            s.w_no_indent("},");
        }

        s.wln_no_indent("];");
    }
}

fn const_default_values(s: &mut Writer, default_values: &Values) {
    for ((value, integer_size), const_name) in default_values {
        let ty_name = if let Some(t) = integer_size {
            t.string_value()
        } else {
            value.const_variable_type_name()
        };

        s.wln(get_const_definition(
            const_name,
            ty_name,
            &value.to_string_value(),
        ));
    }

    s.newline();
}

fn get_const_definition(const_name: &str, ty_name: &str, value: &str) -> String {
    format!("const {const_name}:{ty_name}={value};")
}
