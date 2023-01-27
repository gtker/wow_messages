use crate::base_printer::data::items::Field;
use crate::base_printer::writer::Writer;
use crate::base_printer::{Expansion, ImportFrom};
use std::collections::BTreeSet;

pub(crate) fn definition(s: &mut Writer, v: &[Field], expansion: Expansion) {
    includes(s, v, expansion, ImportFrom::Base);

    struct_definition(s, v);

    impl_block(s, v);
}

pub(crate) fn includes(s: &mut Writer, v: &[Field], expansion: Expansion, location: ImportFrom) {
    let mut set = BTreeSet::new();
    if location == ImportFrom::Items {
        set.insert("Item");
    }

    let location = match location {
        ImportFrom::Items => "wow_world_base",
        ImportFrom::Base => "crate",
    };
    s.wln(format!(
        "use {location}::{}::{{",
        expansion.as_module_string()
    ));
    s.inc_indent();

    for e in v {
        if e.value.should_import() {
            set.insert(e.value.type_name());
        }
    }

    for e in set {
        s.wln(format!("{},", e));
    }

    s.dec_indent();
    s.wln("};");
    s.newline();
}

fn struct_definition(s: &mut Writer, v: &[Field]) {
    s.wln("#[derive(Debug, Copy, Clone)]");
    s.open_curly("pub struct Item");

    for e in v {
        s.wln(format!("pub {}: {},", e.name, e.value.type_name()));
    }

    s.closing_curly();
    s.newline();
}

fn impl_block(s: &mut Writer, v: &[Field]) {
    s.open_curly("impl Item");
    s.wln("#[allow(clippy::complexity)]");

    s.pub_const_fn_new(
        |s| {
            for e in v {
                s.wln(format!("{}: {},", e.name, e.value.type_name()));
            }
        },
        |s| {
            for e in v {
                s.wln(format!("{},", e.name));
            }
        },
    );

    s.closing_curly();
}
