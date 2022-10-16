use std::collections::BTreeSet;
use std::fmt::Write;

use crate::file_utils::get_import_path;
use crate::parser::types::version::Version;
use crate::parser::types::version::{LoginVersion, WorldVersion};
use crate::Objects;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub(crate) struct ObjectTags {
    login_versions: BTreeSet<LoginVersion>,
    world_versions: BTreeSet<WorldVersion>,
    description: Option<TagString>,
    compressed: Option<String>,
    comment: Option<TagString>,
    display: Option<String>,
    paste_versions: BTreeSet<WorldVersion>,

    skip_serialize: Option<bool>,
    is_test: Option<bool>,
    skip: Option<bool>,
    unimplemented: Option<bool>,
    rust_base_ty: Option<bool>,
}

impl ObjectTags {
    pub(crate) fn from_parsed(
        login_versions: BTreeSet<LoginVersion>,
        world_versions: BTreeSet<WorldVersion>,
        description: Option<TagString>,
        compressed: Option<String>,
        comment: Option<TagString>,
        display: Option<String>,
        paste_versions: BTreeSet<WorldVersion>,

        skip_serialize: Option<bool>,
        is_test: Option<bool>,
        skip: Option<bool>,
        unimplemented: Option<bool>,
        rust_base_ty: Option<bool>,
    ) -> Self {
        Self {
            login_versions,
            world_versions,
            description,
            compressed,
            comment,
            display,
            paste_versions,
            skip_serialize,
            is_test,
            skip,
            unimplemented,
            rust_base_ty,
        }
    }

    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn new_with_version(version: Version) -> Self {
        let mut s = Self::new();
        match version {
            Version::Login(l) => s.login_versions.insert(l),
            Version::World(l) => s.world_versions.insert(l),
        };
        s
    }

    pub(crate) fn push_version(&mut self, v: WorldVersion) {
        self.world_versions.insert(v);
    }

    pub(crate) fn paste_versions(&self) -> Vec<WorldVersion> {
        self.paste_versions.clone().into_iter().collect()
    }

    pub(crate) fn unimplemented(&self) -> bool {
        if let Some(v) = self.unimplemented {
            v
        } else {
            false
        }
    }

    pub(crate) fn shared(&self) -> bool {
        self.main_versions().count() != 1
    }

    /// self and tags have any version in common at all
    pub(crate) fn has_version_intersections(&self, tags: &ObjectTags) -> bool {
        if tags.test() && self.test() {
            return true;
        }

        if ((tags.has_login_version() && self.has_wildcard_logon_version())
            || (self.has_login_version() && tags.has_wildcard_logon_version()))
            || ((tags.has_world_version() && self.has_wildcard_world_version())
                || (self.has_world_version() && tags.has_wildcard_world_version()))
        {
            return true;
        }

        for t in self.logon_versions() {
            if tags.logon_versions().any(|a| a == t) {
                return true;
            }
        }

        for outer in self.versions() {
            for inner in tags.versions() {
                if outer.overlaps(&inner) {
                    return true;
                }
            }
        }

        false
    }

    pub(crate) fn is_main_version(&self) -> bool {
        let mut versions = ObjectTags::new_with_version(Version::World(WorldVersion::Minor(1, 12)));
        versions.push_version(WorldVersion::Patch(2, 4, 3));
        versions.push_version(WorldVersion::Patch(3, 3, 5));

        let logon = ObjectTags::new_with_version(Version::Login(LoginVersion::All));

        self.has_version_intersections(&versions) || self.has_version_intersections(&logon)
    }

    pub(crate) fn has_wildcard_logon_version(&self) -> bool {
        self.logon_versions().any(|a| a == LoginVersion::All)
    }

    pub(crate) fn has_wildcard_world_version(&self) -> bool {
        self.versions().any(|a| a == WorldVersion::All)
    }

    pub(crate) fn logon_versions(&self) -> impl Iterator<Item = LoginVersion> {
        self.login_versions.clone().into_iter()
    }

    pub(crate) fn has_logon_versions(&self) -> bool {
        self.logon_versions().next().is_some()
    }

    pub(crate) fn versions(&self) -> impl Iterator<Item = WorldVersion> {
        self.world_versions.clone().into_iter()
    }

    pub(crate) fn has_world_version(&self) -> bool {
        self.versions().next().is_some()
    }

    pub(crate) fn main_versions(&self) -> impl Iterator<Item = Version> + '_ {
        let world = self
            .versions()
            .filter(|a| a.is_main_version())
            .map(Version::World);

        self.logon_versions().map(Version::Login).chain(world)
    }

    pub(crate) fn first_and_main_versions(&self) -> (Version, Vec<Version>) {
        let mut v = self.main_versions();
        let first = v.next().unwrap();

        let rest = v.collect::<Vec<_>>();

        (first, rest)
    }

    pub(crate) fn import_version(&self) -> Version {
        let (first, _) = self.first_and_main_versions();

        first
    }

    /// self is able to fulfill all version obligations for tags
    pub(crate) fn fulfills_all(&self, tags: &Self) -> bool {
        for version in tags.logon_versions() {
            if !self.logon_versions().any(|a| a == version)
                && !self.logon_versions().any(|a| a == LoginVersion::All)
            {
                return false;
            }
        }

        for version in tags.versions() {
            let mut covered = false;

            for self_version in self.versions() {
                if self_version.covers(&version) {
                    covered = true;
                }
            }

            if !covered {
                return false;
            }
        }

        true
    }

    pub(crate) fn has_login_version(&self) -> bool {
        if !self.login_versions.is_empty() {
            assert!(self.world_versions.is_empty());
            return true;
        } else if !self.world_versions.is_empty() {
            assert!(self.login_versions.is_empty());
            return false;
        }

        false
    }

    pub(crate) fn description(&self) -> Option<&TagString> {
        self.description.as_ref()
    }

    pub(crate) fn compressed(&self) -> Option<&String> {
        self.compressed.as_ref()
    }

    pub(crate) fn comment(&self) -> Option<&TagString> {
        self.comment.as_ref()
    }

    pub(crate) fn display(&self) -> Option<&str> {
        if let Some(v) = &self.display {
            Some(v.as_str())
        } else {
            None
        }
    }

    pub(crate) fn is_in_base(&self) -> bool {
        if let Some(v) = self.rust_base_ty {
            v
        } else {
            false
        }
    }

    pub(crate) fn skip(&self) -> bool {
        if let Some(v) = self.skip {
            v
        } else {
            false
        }
    }

    pub(crate) fn test(&self) -> bool {
        if let Some(v) = self.is_test {
            v
        } else {
            false
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum TagStringSymbol {
    Text(String),
    Link(String),
    Newline,
}

impl TagStringSymbol {
    fn from_str(s: &str) -> Vec<Self> {
        if !s.contains('[') {
            return vec![TagStringSymbol::Text(s.to_string())];
        }

        let mut v = Vec::new();
        let mut s = s.to_string();

        while let Some(start) = s.find('[') {
            let end = s.find(']').unwrap();
            v.push(TagStringSymbol::Text(s[..start].to_string()));
            v.push(TagStringSymbol::Link(s[start + 1..end].to_string()));

            s = s[end + 1..].to_string();
        }
        if !s.is_empty() {
            v.push(TagStringSymbol::Text(s));
        }

        v
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub(crate) struct TagString {
    inner: Vec<TagStringSymbol>,
}

impl TagString {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add(&mut self, s: &str) {
        if !self.inner.is_empty() {
            self.inner.push(TagStringSymbol::Newline);
        }

        self.inner.append(&mut TagStringSymbol::from_str(s));
    }

    pub(crate) fn as_doc_lines(&self) -> Vec<String> {
        let mut v = Vec::new();

        let mut current = String::new();
        for i in &self.inner {
            match i {
                TagStringSymbol::Text(s) => current.push_str(s),
                TagStringSymbol::Link(s) => write!(
                    current,
                    "[{name}](./{lower}.md)",
                    name = s,
                    lower = s.to_lowercase()
                )
                .unwrap(),
                TagStringSymbol::Newline => {
                    v.push(current.clone());
                    current.clear();
                }
            }
        }

        v.push(current);

        v
    }

    pub(crate) fn as_rust_doc_lines(&self, o: &Objects, object_tags: &ObjectTags) -> Vec<String> {
        let mut v = Vec::new();

        let mut current = String::new();

        for i in &self.inner {
            match i {
                TagStringSymbol::Text(s) => current.push_str(s),
                TagStringSymbol::Link(s) => {
                    if let Some(tags) = o.get_tags_of_object_fallible(s, object_tags) {
                        let version = tags.import_version();
                        write!(current, "[`{}`]({}::{})", s, get_import_path(version), s).unwrap()
                    } else {
                        write!(current, "`{}`", s).unwrap()
                    }
                }
                TagStringSymbol::Newline => {
                    v.push(current.clone());
                    current.clear();
                }
            }
        }

        v.push(current);

        v
    }

    pub(crate) fn as_doc_table_string(&self) -> String {
        let mut s = String::new();

        for i in &self.inner {
            match i {
                TagStringSymbol::Text(t) => s.push_str(t),
                TagStringSymbol::Link(l) => {
                    write!(s, "[{}](./{}.md)", l, l.to_lowercase()).unwrap()
                }
                TagStringSymbol::Newline => s.push_str("<br/>"),
            }
        }

        s
    }

    pub(crate) fn as_ir_string(&self) -> String {
        let mut s = String::new();

        for i in &self.inner {
            match i {
                TagStringSymbol::Text(t) => s.push_str(t),
                TagStringSymbol::Link(l) => write!(s, "[{}]", l).unwrap(),
                TagStringSymbol::Newline => s.push('\n'),
            }
        }

        s
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub(crate) struct MemberTags {
    description: Option<TagString>,
    compressed: Option<String>,
    comment: Option<TagString>,
    display: Option<String>,

    skip_serialize: Option<bool>,
}

impl MemberTags {
    pub(crate) fn from_parsed(
        description: Option<TagString>,
        compressed: Option<String>,
        comment: Option<TagString>,
        display: Option<String>,
        skip_serialize: Option<bool>,
    ) -> Self {
        Self {
            description,
            compressed,
            comment,
            display,
            skip_serialize,
        }
    }

    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn description(&self) -> Option<&TagString> {
        self.description.as_ref()
    }

    pub(crate) fn compressed(&self) -> Option<&String> {
        self.compressed.as_ref()
    }

    pub(crate) fn is_compressed(&self) -> bool {
        self.compressed.is_some()
    }

    pub(crate) fn skip_serialize(&self) -> bool {
        self.skip_serialize.unwrap_or(false)
    }

    pub(crate) fn comment(&self) -> Option<&TagString> {
        self.comment.as_ref()
    }

    pub(crate) fn display(&self) -> Option<&str> {
        if let Some(v) = &self.display {
            Some(v.as_str())
        } else {
            None
        }
    }
}
