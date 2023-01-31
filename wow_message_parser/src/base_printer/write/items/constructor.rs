use crate::base_printer::data::items::{Array, Field};
use crate::base_printer::write::items::definition::includes;
use crate::base_printer::writer::Writer;
use crate::base_printer::{Expansion, ImportFrom};

pub(crate) fn constructor(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    expansion: Expansion,
) {
    includes(
        s,
        &fields,
        &arrays,
        expansion,
        ImportFrom::ItemsConstructors,
    );

    s.constructor(
        "n",
        |s| {
            for e in fields {
                s.wln(format!("{}: {},", e.name, e.value.type_name()));
            }

            for array in arrays {
                for instance in &array.instances {
                    for field in instance {
                        s.wln(format!(
                            "{}: {},",
                            field.variable_name,
                            field.value.type_name()
                        ));
                    }
                }
            }
        },
        |s| {
            for e in fields {
                s.wln(format!("{},", e.name));
            }

            for array in arrays {
                s.wln(format!("{}: [", array.variable_name));

                for instance in &array.instances {
                    if array.import_only {
                        s.open_curly(format!("{}", array.type_name));

                        for field in instance {
                            s.wln(format!("{}: {},", field.name, field.variable_name,));
                        }

                        s.dec_indent();
                        s.wln("},");
                    } else {
                        s.wln(format!("{}::new(", array.type_name));
                        for field in instance {
                            s.wln(format!("{},", field.variable_name,));
                        }
                        s.wln("),");
                    }
                }

                s.wln("],");
            }
        },
    );
}
