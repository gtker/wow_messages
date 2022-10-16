use std::collections::BTreeSet;
use std::fmt::Write;

use crate::file_utils::{get_import_path, get_shared_module_name};
use crate::parser::types::version::{AllVersions, Version};
use crate::parser::types::version::{LoginVersion, WorldVersion};
use crate::Objects;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ObjectTags {
    all_versions: AllVersions,

    description: Option<TagString>,
    comment: Option<TagString>,

    compressed: bool,
    is_test: bool,
    skip: bool,
    unimplemented: bool,
    rust_base_ty: bool,
}

impl ObjectTags {
    pub(crate) fn from_parsed(
        all_versions: AllVersions,
        description: Option<TagString>,
        comment: Option<TagString>,
        compressed: bool,
        is_test: bool,
        skip: bool,
        unimplemented: bool,
        rust_base_ty: bool,
    ) -> Self {
        Self {
            all_versions,
            description,
            compressed,
            comment,
            is_test,
            skip,
            unimplemented,
            rust_base_ty,
        }
    }

    pub(crate) fn new_with_version(version: Version) -> Self {
        let v = match version {
            Version::Login(l) => {
                let mut s = BTreeSet::new();
                s.insert(l);
                (AllVersions::Login(s.clone()), s, BTreeSet::new())
            }
            Version::World(l) => {
                let mut s = BTreeSet::new();
                s.insert(l);
                (AllVersions::World(s.clone()), BTreeSet::new(), s)
            }
        };

        Self {
            all_versions: v.0,
            description: None,
            comment: None,
            compressed: false,
            is_test: false,
            skip: false,
            unimplemented: false,
            rust_base_ty: false,
        }
    }

    pub(crate) fn push_version(&mut self, v: WorldVersion) {
        match &mut self.all_versions {
            AllVersions::Login(_) => unreachable!(),
            AllVersions::World(w) => w.insert(v),
        };
    }

    pub(crate) fn unimplemented(&self) -> bool {
        self.unimplemented
    }

    pub(crate) fn shared(&self) -> bool {
        self.main_versions().count() != 1
    }

    /// self and tags have any version in common at all
    pub(crate) fn has_version_intersections(&self, tags: &ObjectTags) -> bool {
        self.all_versions
            .has_version_intersections(&tags.all_versions)
    }

    pub(crate) fn is_main_version(&self) -> bool {
        let mut versions = ObjectTags::new_with_version(Version::World(WorldVersion::Minor(1, 12)));
        versions.push_version(WorldVersion::Patch(2, 4, 3));
        versions.push_version(WorldVersion::Patch(3, 3, 5));

        let logon = ObjectTags::new_with_version(Version::Login(LoginVersion::All));

        self.has_version_intersections(&versions) || self.has_version_intersections(&logon)
    }

    pub(crate) fn logon_versions(&self) -> impl Iterator<Item = LoginVersion> {
        match &self.all_versions {
            AllVersions::Login(i) => i.clone().into_iter(),
            AllVersions::World(_) => BTreeSet::new().into_iter(),
        }
    }

    pub(crate) fn versions(&self) -> impl Iterator<Item = WorldVersion> {
        match &self.all_versions {
            AllVersions::World(w) => w.clone().into_iter(),
            AllVersions::Login(_) => BTreeSet::new().into_iter(),
        }
    }

    pub(crate) fn has_world_version(&self) -> bool {
        matches!(self.all_versions, AllVersions::World(_))
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

    pub(crate) fn shared_module_name(&self, object_name: &str) -> String {
        let versions = self
            .main_versions()
            .map(|a| a.as_world())
            .collect::<Vec<_>>();

        get_shared_module_name(object_name, &versions)
    }

    /// self is able to fulfill all version obligations for tags
    pub(crate) fn fulfills_all(&self, tags: &Self) -> bool {
        self.all_versions.fulfills_all(&tags.all_versions)
    }

    pub(crate) fn has_login_version(&self) -> bool {
        matches!(self.all_versions, AllVersions::Login(_))
    }

    pub(crate) fn description(&self) -> Option<&TagString> {
        self.description.as_ref()
    }

    pub(crate) fn comment(&self) -> Option<&TagString> {
        self.comment.as_ref()
    }

    pub(crate) fn compressed(&self) -> bool {
        self.compressed
    }

    pub(crate) fn is_in_base(&self) -> bool {
        self.rust_base_ty
    }

    pub(crate) fn skip(&self) -> bool {
        self.skip
    }

    pub(crate) fn test(&self) -> bool {
        self.is_test
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
