use std::collections::BTreeSet;
use std::fmt::Write;

use crate::file_utils::{get_import_path, get_shared_module_name};
use crate::parser::types::version::{
    AllRustVersions, AllVersions, MajorWorldVersion, Version, TBC, VANILLA, WRATH,
};
use crate::parser::types::version::{LoginVersion, WorldVersion};
use crate::Objects;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub(crate) struct ObjectTags {
    all_versions: AllVersions,
    rust_versions: Option<AllRustVersions>,

    comment: Option<TagString>,

    compressed: bool,
    is_test: bool,
    skip: bool,
    unimplemented: bool,
    rust_base_ty: bool,
    zero_is_always_valid: bool,
    non_network_type: bool,
    used_in_update_mask: bool,
}

impl ObjectTags {
    pub(crate) fn from_parsed(
        all_versions: AllVersions,
        comment: Option<TagString>,
        compressed: bool,
        is_test: bool,
        skip: bool,
        unimplemented: bool,
        rust_base_ty: bool,
        zero_is_always_valid: bool,
        non_network_type: bool,
        used_in_update_mask: bool,
    ) -> Self {
        let rust_versions = match &all_versions {
            AllVersions::Login(l) => Some(AllRustVersions::Login(l.clone())),
            AllVersions::World(l) => {
                let mut b = BTreeSet::new();
                if l.contains(&WorldVersion::All) {
                    b.insert(MajorWorldVersion::Vanilla);
                    b.insert(MajorWorldVersion::BurningCrusade);
                    b.insert(MajorWorldVersion::Wrath);
                }

                for v in l {
                    if let Some(v) = v.try_as_major_world() {
                        b.insert(v);
                    }
                }

                if b.is_empty() {
                    None
                } else {
                    Some(AllRustVersions::World(b))
                }
            }
        };

        Self {
            all_versions,
            rust_versions,
            compressed,
            comment,
            is_test,
            skip,
            unimplemented,
            rust_base_ty,
            zero_is_always_valid,
            non_network_type,
            used_in_update_mask,
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
                s.insert(l.as_world());
                (AllVersions::World(s.clone()), BTreeSet::new(), s)
            }
        };

        Self::from_parsed(
            v.0, None, false, false, false, false, false, false, false, false,
        )
    }

    pub(crate) fn new_with_world_versions(versions: &[MajorWorldVersion]) -> Self {
        let mut s = BTreeSet::new();
        for v in versions {
            s.insert(v.as_world());
        }

        Self::from_parsed(
            AllVersions::World(s),
            None,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
        )
    }

    pub(crate) fn unimplemented(&self) -> bool {
        self.unimplemented
    }

    pub(crate) fn non_network_type(&self) -> bool {
        self.non_network_type
    }

    pub(crate) fn shared(&self) -> bool {
        self.main_versions().count() != 1
    }

    pub(crate) fn only_main_world_version(&self) -> MajorWorldVersion {
        match self.first_and_main_versions().0 {
            Version::Login(_) => panic!(),
            Version::World(w) => w,
        }
    }

    pub(crate) fn all_versions(&self) -> &AllVersions {
        &self.all_versions
    }

    pub(crate) fn all_rust_versions(&self) -> &AllRustVersions {
        self.rust_versions.as_ref().unwrap()
    }

    pub(crate) fn try_all_rust_versions(&self) -> Option<&AllRustVersions> {
        self.rust_versions.as_ref()
    }

    /// self and tags have any version in common at all
    pub(crate) fn has_version_intersections(&self, tags: &ObjectTags) -> bool {
        self.all_versions
            .has_version_intersections(&tags.all_versions)
    }

    pub(crate) fn has_rust_version(&self) -> bool {
        self.rust_versions.is_some()
    }

    pub(crate) fn logon_versions(&self) -> impl Iterator<Item = LoginVersion> {
        match &self.all_versions {
            AllVersions::Login(i) => i.clone().into_iter(),
            AllVersions::World(_) => BTreeSet::new().into_iter(),
        }
    }

    pub(crate) fn world_versions(&self) -> impl Iterator<Item = WorldVersion> {
        match &self.all_versions {
            AllVersions::World(w) => {
                if w.contains(&WorldVersion::All) {
                    let mut w = BTreeSet::new();
                    w.insert(VANILLA);
                    w.insert(TBC);
                    w.insert(WRATH);
                    w.into_iter()
                } else {
                    w.clone().into_iter()
                }
            }
            AllVersions::Login(_) => BTreeSet::new().into_iter(),
        }
    }

    pub(crate) fn has_world_version(&self) -> bool {
        matches!(self.all_versions, AllVersions::World(_))
    }

    pub(crate) fn main_versions(&self) -> impl Iterator<Item = Version> + '_ {
        let world = self
            .world_versions()
            .filter_map(|a| a.try_as_major_world())
            .map(Version::World);

        self.logon_versions().map(Version::Login).chain(world)
    }

    pub(crate) fn contains_wrath(&self) -> bool {
        self.main_versions().any(|a| match a {
            Version::Login(_) => false,
            Version::World(w) => w.wrath_or_greater(),
        })
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
            .map(|a| a.as_major_world())
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

    pub(crate) fn has_login_version_8_or_all(&self) -> bool {
        match &self.all_versions {
            AllVersions::Login(l) => {
                l.contains(&LoginVersion::Specific(8)) || l.contains(&LoginVersion::All)
            }
            AllVersions::World(_) => false,
        }
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
    pub(crate) fn zero_is_always_valid(&self) -> bool {
        self.zero_is_always_valid
    }

    pub(crate) fn used_in_update_mask(&self) -> bool {
        self.used_in_update_mask
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
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
                        if tags.is_in_base() == object_tags.is_in_base() {
                            let version = tags.import_version();
                            write!(current, "[`{}`]({}::{})", s, get_import_path(version), s)
                                .unwrap()
                        } else {
                            write!(current, "`{s}`").unwrap()
                        }
                    } else {
                        write!(current, "`{s}`").unwrap()
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
                TagStringSymbol::Link(l) => write!(s, "[{l}]").unwrap(),
                TagStringSymbol::Newline => s.push('\n'),
            }
        }

        s
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub(crate) struct MemberTags {
    comment: Option<TagString>,
    display: Option<String>,
    valid_range: Option<(i128, i128)>,
    maximum_length: Option<i128>,
}

impl MemberTags {
    pub(crate) fn from_parsed(
        comment: Option<TagString>,
        display: Option<String>,
        valid_range: Option<(i128, i128)>,
        maximum_length: Option<i128>,
    ) -> Self {
        Self {
            comment,
            display,
            valid_range,
            maximum_length,
        }
    }

    pub(crate) fn new() -> Self {
        Self::default()
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

    pub(crate) fn valid_range(&self) -> Option<(i128, i128)> {
        self.valid_range
    }

    pub(crate) fn maximum_length(&self) -> Option<i128> {
        self.maximum_length
    }
}
