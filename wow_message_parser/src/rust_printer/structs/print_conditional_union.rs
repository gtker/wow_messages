use crate::parser::types::container::Container;
use crate::parser::types::objects::Objects;
use crate::rust_printer::print_serde_derive;
use crate::rust_printer::rust_view::conditional_union::{ConditionalUnion, ConditionalUnionBranch};
use crate::rust_printer::structs::{print_derives, print_member_docc_description_and_comment};
use crate::rust_printer::writer::Writer;

pub(crate) fn print_declaration(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    union: &ConditionalUnion,
) {
    print_record_declaration(s, e, o, union.linear());
    print_record_declaration(s, e, o, union.full());

    let members: Vec<_> = union
        .linear()
        .object()
        .all_members()
        .into_iter()
        .chain(union.full().object().all_members())
        .collect();
    print_derives(s, &members, true);
    print_serde_derive(s, e.tags().is_in_base(), false);
    s.new_enum("pub", e.name(), |s| {
        s.wln(format!(
            "{}({}),",
            union.linear().variant_name(),
            union.linear().record_name()
        ));
        s.wln(format!(
            "{}({}),",
            union.full().variant_name(),
            union.full().record_name()
        ));
    });

    s.bodyn(format!("impl Default for {}", e.name()), |s| {
        s.body("fn default() -> Self", |s| {
            s.wln(format!(
                "Self::{}(Default::default())",
                union.linear().variant_name()
            ));
        });
    });
}

fn print_record_declaration(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    branch: &ConditionalUnionBranch,
) {
    let members = branch.object().all_members();
    print_derives(s, &members, false);
    print_serde_derive(s, e.tags().is_in_base(), false);
    s.new_struct(branch.record_name(), |s| {
        for member in branch.object().members_in_struct() {
            print_member_docc_description_and_comment(s, member.tags(), o, e.tags());
            s.wln(format!(
                "pub {}: {},",
                member.name(),
                member.ty().rust_str()
            ));
        }
    });
}
