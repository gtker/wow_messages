mod bytes;
mod members;

use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::version::AllVersions;
use crate::rust_printer::writer::Writer;
use crate::rust_printer::CFG_TESTCASE;

fn wln(s: &mut Writer, msg: impl AsRef<str>) {
    let msg = msg.as_ref();

    s.wln(format!("writeln!(s, \"{msg}\").unwrap();"));
}

fn wlna(s: &mut Writer, msg: impl AsRef<str>, args: impl AsRef<str>) {
    let msg = msg.as_ref();
    let args = args.as_ref();

    s.wln(format!("writeln!(s, \"{msg}\", {args}).unwrap();"));
}

pub(crate) fn print_to_testcase(s: &mut Writer, e: &Container, should_print_body: bool) {
    if !should_print_body {
        return;
    }

    s.wln(CFG_TESTCASE);
    s.bodyn("fn to_test_case_string(&self) -> Option<String>", |s| {
        print_inner_function(s, e);
    });
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
    match e.tags().all_versions() {
        AllVersions::Login(l) => {
            let versions = if let Ok(versions) = std::env::var("WOWM_TEST_CASE_LOGIN_VERSION") {
                versions
            } else {
                l.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            };

            wlna(
                s,
                "    login_versions = \\\"{}\\\";",
                format!("std::env::var(\"WOWM_TEST_CASE_LOGIN_VERSION\").unwrap_or(\"{versions}\".to_string())"),
            );
        }
        AllVersions::World(l) => {
            let versions = l
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            wlna(
                s,
                "    versions = \\\"{}\\\";",
                format!("std::env::var(\"WOWM_TEST_CASE_WORLD_VERSION\").unwrap_or(\"{versions}\".to_string())"),
            );
        }
    }

    wln(s, "}}\\n");
    s.newline();

    s.wln("Some(s)");
}
