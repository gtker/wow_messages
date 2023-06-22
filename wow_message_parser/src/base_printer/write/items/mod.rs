pub(crate) mod all_items;
pub(crate) mod constructor;
pub(crate) mod conversions;
pub(crate) mod definition;

use crate::base_printer::data::items::{
    Array, ArrayInstances, Field, IntegerSize, Optimizations, Value,
};
use std::collections::BTreeSet;

pub(crate) struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub stamina: i32,
    pub intellect: i32,
    pub spirit: i32,
    pub health: i32,
    pub mana: i32,
}

use crate::base_printer::write::items::all_items::{all_items, lookup_type};
use crate::base_printer::write::items::constructor::constructor;
use crate::base_printer::write::items::definition::{definition, includes};
use crate::base_printer::{Expansion, ImportFrom};
use crate::file_utils::{
    overwrite_autogenerate_if_not_same_contents, overwrite_if_not_same_contents,
};
use crate::rust_printer::writer::Writer;
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

    overwrite_if_not_same_contents(s.inner(), path);
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
    lib_functions(&mut s, ty_name, things);

    overwrite_autogenerate_if_not_same_contents(s.inner(), path);
}

fn lib_functions(s: &mut Writer, ty_name: &str, things: &[GenericThing]) {
    let min = things[0].entry;
    let max = things.iter().last().unwrap().entry;

    let ty_lower = ty_name.to_lowercase();

    s.wln(format!("/// Looks up {ty_lower}s and returns if found."));
    s.wln("///");
    s.wln(format!("/// Prefer using this over [`all_{ty_lower}s`] since this utilizes a lookup array for very fast lookup."));

    s.bodyn(
        format!("pub const fn lookup_{ty_lower}(id: u32) -> Option<&'static {ty_name}>"),
        |s| {
            s.bodyn(format!("if id < {min} || id > {max}"), |s| {
                s.wln("return None;");
            });

            let max_index = things.len();
            let ty = lookup_type(max_index);
            s.wln(format!(
                "let index = data::Z________LOOKUP[(id - {min}) as usize];"
            ));
            s.body_else(
                format!("if index == {ty}::MAX || index as usize > (all_{ty_lower}s().len() - 1)"),
                |s| {
                    s.wln("None");
                },
                |s| {
                    s.wln(format!("Some(&all_{ty_lower}s()[index as usize])"));
                },
            );
        },
    );

    s.wln(format!("/// Returns all {ty_lower}s."));
    s.wln("///");
    s.wln(format!("/// Prefer using [`lookup_{ty_lower}`] since it incorporates optimizations for lookup speed."));
    s.bodyn(
        format!("pub const fn all_{ty_lower}s() -> &'static [{ty_name}]"),
        |s| {
            s.wln("data::Z________DATA");
        },
    );

    s.wln("#[cfg(test)]");
    s.body("mod test", |s| {
        s.wln(format!("use super::lookup_{ty_lower};"));
        s.newline();

        s.wln("#[test]");
        s.body("fn tests()", |s| {
            s.wln(format!("assert!(lookup_{ty_lower}(u32::MIN).is_none());"));
            s.wln(format!("assert!(lookup_{ty_lower}(u32::MAX).is_none());"));
            s.newline();

            s.wln(format!("const MIN: u32 = {min};"));
            s.wln(format!("const MAX: u32 = {max};"));
            s.wln(format!(
                "assert_eq!(lookup_{ty_lower}(MIN).unwrap().entry(), MIN);"
            ));
            s.wln(format!(
                "assert_eq!(lookup_{ty_lower}(MAX).unwrap().entry(), MAX);"
            ));
            s.newline();

            s.wln(format!("assert!(lookup_{ty_lower}(MIN - 1).is_none());"));
            s.wln(format!("assert!(lookup_{ty_lower}(MAX + 1).is_none());"));
        });
    });
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

    overwrite_if_not_same_contents(s.inner(), path);
}

pub(crate) fn write_things(
    path: &Path,
    things: &[GenericThing],
    expansion: Expansion,
    ty_name: &str,
    optimizations: &Optimizations,
) {
    let mut s = Writer::new();

    let (default_values, arrays) = get_default_values(things, optimizations);
    let mut sorted = BTreeSet::new();
    const_default_values(&default_values, &mut sorted);
    print_arrays(&arrays, &mut sorted);

    for line in sorted {
        s.wln(line);
    }

    all_items(
        &mut s,
        things,
        expansion,
        ty_name,
        &default_values,
        &arrays,
        optimizations,
    );

    overwrite_if_not_same_contents(s.inner(), path);
}

struct ConstNamer {
    current: String,
    alphabet_index: usize,
}

impl ConstNamer {
    const ALPHABET: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    pub fn new() -> Self {
        Self {
            current: "".to_string(),
            alphabet_index: 0,
        }
    }

    fn increment_index(&mut self) {
        self.alphabet_index += 1;

        if self.alphabet_index >= Self::ALPHABET.len() {
            self.alphabet_index = 0;

            if self.current.is_empty() {
                self.current.push(Self::ALPHABET[self.alphabet_index]);
            } else {
                let len = self.current.len();

                let mut id = self.char_index();
                while id + 1 >= Self::ALPHABET.len() {
                    if self.current.is_empty() {
                        for _ in 0..len + 1 {
                            self.current.push(Self::ALPHABET[0]);
                        }
                        return;
                    }

                    id = self.char_index();
                }

                self.current.push(Self::ALPHABET[id + 1]);

                while self.current.len() < len {
                    self.current.push(Self::ALPHABET[0]);
                }
            }
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

    fn char_index(&mut self) -> usize {
        let char = self.current.pop().unwrap();

        Self::ALPHABET
            .iter()
            .enumerate()
            .find(|(_, a)| a == &&char)
            .unwrap()
            .0
    }
}

#[cfg(test)]
mod test {
    use crate::base_printer::write::items::ConstNamer;

    #[test]
    fn namer() {
        let mut n = ConstNamer::new();

        assert_eq!("A", n.next());

        for _ in 0..25 {
            let _ = n.next();
        }

        assert_eq!("AA", n.next());
        assert_eq!("AB", n.next());

        for _ in 0..24 {
            let _ = n.next();
        }

        assert_eq!("BA", n.next());
        assert_eq!("BB", n.next());

        for _ in 0..24 {
            let _ = n.next();
        }

        assert_eq!("CA", n.next());
        assert_eq!("CB", n.next());

        for _ in 0..620 {
            let _ = n.next();
        }

        assert_eq!("ZY", n.next());
        assert_eq!("ZZ", n.next());
        assert_eq!("AAA", n.next());

        for _ in 0..24 {
            let _ = n.next();
        }

        assert_eq!("AAZ", n.next());
        assert_eq!("ABA", n.next());

        for _ in 0..620 {
            let _ = n.next();
        }

        assert_eq!("AYX", n.next());
        assert_eq!("AYY", n.next());
        assert_eq!("AYZ", n.next());

        for _ in 0..24 {
            let _ = n.next();
        }

        assert_eq!("AZY", n.next());
        assert_eq!("AZZ", n.next());
        assert_eq!("BAA", n.next());
    }
}

type Values = HashMap<(Value, Option<IntegerSize>), String>;
type Arrays<'a> = HashMap<(&'a ArrayInstances, &'static str), String>;

fn get_default_values<'a>(
    things: &'a [GenericThing],
    optimizations: &Optimizations,
) -> (Values, Arrays<'a>) {
    let mut values: HashMap<(Value, Option<IntegerSize>), usize> = HashMap::new();
    let mut arrays = HashMap::new();
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

fn print_arrays(arrays: &Arrays, sorted: &mut BTreeSet<String>) {
    for ((array, ty_name), const_name) in arrays {
        let mut s = Writer::new();
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

        s.w_no_indent("];");
        sorted.insert(s.into_inner());
    }
}

fn const_default_values(default_values: &Values, sorted: &mut BTreeSet<String>) {
    for ((value, integer_size), const_name) in default_values {
        let ty_name = if let Some(t) = integer_size {
            t.string_value()
        } else {
            value.const_variable_type_name()
        };

        sorted.insert(get_const_definition(
            const_name,
            ty_name,
            &value.to_string_value(),
        ));
    }
}

fn get_const_definition(const_name: &str, ty_name: &str, value: &str) -> String {
    format!("const {const_name}:{ty_name}={value};")
}
