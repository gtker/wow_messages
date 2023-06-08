mod bytes;
mod members;

use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::objects::Objects;
use crate::rust_printer::writer::Writer;
use crate::rust_printer::CFG_TESTCASE;

/// Write LiNe
fn wln(s: &mut Writer, msg: impl AsRef<str>) {
    let msg = msg.as_ref();

    s.wln(format!("writeln!(s, \"{msg}\").unwrap();"));
}

/// Write LiNe Argument
fn wlna(s: &mut Writer, msg: impl AsRef<str>, args: impl AsRef<str>) {
    let msg = msg.as_ref();
    let args = args.as_ref();

    s.wln(format!("writeln!(s, \"{msg}\", {args}).unwrap();"));
}

pub(crate) fn print_to_testcase(s: &mut Writer, e: &Container, should_print_body: bool) {
    if !should_print_body {
        return;
    }

    let name = e.name();
    s.wln(CFG_TESTCASE);
    s.open_curly(format!("impl {name}"));

    s.funcn_pub("to_test_case_string(&self)", "Option<String>", |s| {
        if should_print_body {
            print_inner_function(s, e);
        } else {
            s.wln("None");
        }
    });

    s.closing_curly_newline();
}

fn print_inner_function(s: &mut Writer, e: &Container) {
    if matches!(e.container_type(), ContainerType::Msg(_)) {
        s.wln("panic!(\"MSG types not supported\");");
        return;
    }

    let name = e.name();

    s.wln("use std::fmt::Write;");
    if matches!(
        e.container_type(),
        ContainerType::SMsg(_) | ContainerType::CMsg(_) | ContainerType::Msg(_)
    ) {
        s.wln("use crate::traits::Message;");
    }
    s.newline();

    s.wln("let mut s = String::new();");
    s.newline();

    wln(s, &format!("test {name} {{{{"));
    members::print_members(s, e, "self.", "    ");

    wln(s, "}} [");
    bytes::print_bytes(s, e);

    wln(s, "] {{");
    if e.tags().has_login_version() {
        let versions = if let Ok(versions) = std::env::var("WOWM_TEST_CASE_LOGIN_VERSION") {
            versions
        } else {
            e.tags()
                .logon_versions()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        };

        wlna(
            s,
            &"    login_versions = \\\"{}\\\";",
            format!("std::env::var(\"WOWM_TEST_CASE_LOGIN_VERSION\").unwrap_or(\"{versions}\".to_string())"),
        );
    } else {
        let versions = e
            .tags()
            .versions()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        wlna(
            s,
            &"    versions = \\\"{}\\\";",
            format!("std::env::var(\"WOWM_TEST_CASE_WORLD_VERSION\").unwrap_or(\"{versions}\".to_string())"),
        );
    }
    wln(s, "}}\\n");
    s.newline();

    s.wln("Some(s)");
}
