pub(crate) mod all_items;
pub(crate) mod constructor;
pub(crate) mod conversions;
pub(crate) mod definition;

use crate::base_printer::data::items::{Array, ArrayInstances, Field, Optimizations};
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

    let name_function = match ty_lower.as_str() {
        "item" => "name",
        "spell" => "spell_name",
        _ => unreachable!(),
    };

    s.wln(format!(
        "/// Returns all {ty_lower}s that contain `needle` in the name. The search is case insensitive."
    ));
    s.bodyn(format!("pub fn lookup_{ty_lower}s_by_name(needle: &str) -> impl Iterator<Item = &'static {ty_name}> + '_"), |s| {
        s.body_closing_with(format!("all_{ty_lower}s().iter().filter(move |{ty_lower}|"), |s| {
            s.wln(format!("let lower = {ty_lower}.{name_function}().to_ascii_lowercase();", ty_lower = ty_lower));
            s.wln(format!("lower.contains(needle)"));
        }, ")");
    });

    s.wln(format!(
        "/// Returns the first {ty_lower} that contains `needle` in the name. The search is case insensitive."
    ));
    s.bodyn(
        format!("pub fn lookup_{ty_lower}_by_name(needle: &str) -> Option<&'static {ty_name}>"),
        |s| {
            s.wln("let needle = needle.to_ascii_lowercase();");
            s.newline();

            s.bodyn(format!("for {ty_lower} in all_{ty_lower}s()"), |s| {
                s.wln(format!(
                    "let lower = {ty_lower}.{name_function}().to_ascii_lowercase();"
                ));

                s.bodyn("if lower.contains(&needle)", |s| {
                    s.wln(format!("return Some({ty_lower})"));
                });
            });

            s.wln("None");
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
            s.newline();

            s.body("for i in 0..=MAX + 10", |s| {
                s.body(format!("match lookup_{ty_lower}(i)"), |s| {
                    s.wln("None => {}");
                    s.wln("Some(e) => assert_eq!(i, e.entry()),");
                });
            });
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

    let arrays = get_default_values(things);
    let mut sorted = BTreeSet::new();
    print_arrays(&arrays, &mut sorted);

    for line in sorted {
        s.wln(line);
    }

    all_items(&mut s, things, expansion, ty_name, &arrays, optimizations);

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

type Arrays<'a> = HashMap<(&'a ArrayInstances, &'static str), String>;

fn get_default_values<'a>(things: &'a [GenericThing]) -> Arrays<'a> {
    let mut arrays = HashMap::new();
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

    // Ensure that most used consts have the shortest name
    let mut arrays = arrays.into_iter().collect::<Vec<_>>();

    arrays.sort_by(
        |((value, integer), amount), ((value2, integer2), amount2)| {
            let value = value2.cmp(value);
            let integer = integer2.cmp(integer);

            amount2.cmp(amount).then(value).then(integer)
        },
    );

    let mut namer = ConstNamer::new();
    let mut arrays_output = HashMap::new();
    for (value, _) in arrays {
        arrays_output.insert(value, namer.next());
    }

    arrays_output
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
