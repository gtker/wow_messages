use std::collections::BTreeSet;

use crate::error_printer::{
    object_has_both_versions, object_has_no_versions, version_tags_overlap,
};
use crate::file_info::FileInfo;
use crate::parser::types::tags::{MemberTags, TagString};
use crate::parser::types::version::{AllVersions, LoginVersion, WorldVersion};
use crate::{
    ObjectTags, COMMENT, COMPRESSED, DISPLAY, LOGIN_VERSIONS, MAXIMUM_LENGTH, NON_NETWORK_TYPE,
    PASTE_VERSIONS, RUST_BASE_TYPE, SKIP_STR, TEST_STR, UNIMPLEMENTED, USED_IN_UPDATE_MASK,
    VALID_RANGE, VERSIONS, ZERO_IS_ALWAYS_VALID,
};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub(crate) struct ParsedTags {
    login_versions: BTreeSet<LoginVersion>,
    world_versions: BTreeSet<WorldVersion>,
    compressed: Option<String>,
    comment: Option<TagString>,
    display: Option<String>,
    paste_versions: BTreeSet<WorldVersion>,

    is_test: BoolTag,
    skip: BoolTag,
    unimplemented: BoolTag,
    rust_base_ty: BoolTag,
    zero_is_always_valid: BoolTag,
    non_network_type: BoolTag,
    used_in_update_mask: BoolTag,
    valid_range: Option<(i128, i128)>,
    maximum_length: Option<i128>,
}

impl ParsedTags {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn append(&mut self, mut t: ParsedTags, ty_name: &str, file_info: &FileInfo) {
        self.login_versions.append(&mut t.login_versions);

        for v in t.world_versions {
            self.insert_world_version(v, ty_name, file_info);
        }

        if let Some(v) = t.comment {
            self.comment = Some(v);
        }

        if let Some(v) = t.display {
            self.display = Some(v);
        }

        self.paste_versions.append(&mut t.paste_versions);

        self.is_test.append(t.is_test);
        self.skip.append(t.skip);
        self.unimplemented.append(t.unimplemented);
        self.rust_base_ty.append(t.rust_base_ty);
        self.zero_is_always_valid.append(t.zero_is_always_valid);
        self.non_network_type.append(t.non_network_type);
        self.used_in_update_mask.append(t.used_in_update_mask)
    }

    pub(crate) fn add_descriptive_comments(&mut self, comments: &[&str]) {
        if let Some(v) = &mut self.comment {
            for c in comments {
                v.add(c);
            }
        } else if !comments.is_empty() {
            let mut t = TagString::new();

            for c in comments {
                t.add(c);
            }

            self.comment = Some(t);
        }
    }

    pub(crate) fn compressed(&self) -> bool {
        if let Some(c) = &self.compressed {
            c == "true"
        } else {
            false
        }
    }
    pub(crate) fn into_tags(
        self,
        ty_name: &str,
        file_info: &FileInfo,
        rust_base_type_default: bool,
    ) -> ObjectTags {
        let all_versions = if !self.world_versions.is_empty() {
            if !self.login_versions.is_empty() {
                object_has_both_versions(ty_name, file_info);
            }
            AllVersions::World(self.world_versions)
        } else if !self.login_versions.is_empty() {
            if !self.world_versions.is_empty() {
                object_has_both_versions(ty_name, file_info);
            }
            AllVersions::Login(self.login_versions)
        } else {
            object_has_no_versions(ty_name, file_info)
        };

        let rust_base_type_default =
            rust_base_type_default && matches!(all_versions, AllVersions::World(_));

        ObjectTags::from_parsed(
            all_versions,
            self.comment,
            if let Some(compressed) = self.compressed {
                compressed == "true"
            } else {
                false
            },
            self.is_test.into_bool(),
            self.skip.into_bool(),
            self.unimplemented.into_bool(),
            self.rust_base_ty
                .into_bool_with_default(rust_base_type_default),
            self.zero_is_always_valid.into_bool(),
            self.non_network_type.into_bool(),
            self.used_in_update_mask.into_bool(),
        )
    }

    pub(crate) fn into_member_tags(self) -> MemberTags {
        MemberTags::from_parsed(
            self.comment,
            self.display,
            self.valid_range,
            self.maximum_length,
        )
    }

    pub(crate) fn paste_versions(&self) -> impl Iterator<Item = WorldVersion> {
        self.paste_versions.clone().into_iter()
    }

    pub(crate) fn push_version(&mut self, version: WorldVersion) {
        self.world_versions.insert(version);
    }

    fn insert_world_version(&mut self, version: WorldVersion, ty_name: &str, file_info: &FileInfo) {
        if matches!(version, WorldVersion::All) {
            self.world_versions.clear();
            self.world_versions.insert(WorldVersion::All);
            return;
        }

        if self.world_versions.get(&WorldVersion::All).is_some() {
            panic!("Object has both * and regular version")
        }

        for more_specific in &self.world_versions {
            if version.overlaps(more_specific) {
                version_tags_overlap(ty_name, file_info, *more_specific, version);
            }
        }

        self.world_versions.insert(version);
    }

    pub(crate) fn insert(&mut self, key: &str, value: &str, ty_name: &str, file_info: &FileInfo) {
        if key == LOGIN_VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    if self.world_versions.get(&WorldVersion::All).is_none() {
                        self.login_versions.insert(LoginVersion::Specific(v));
                        continue;
                    } else {
                        continue;
                    }
                } else if w == "*" {
                    self.login_versions.clear();
                    self.login_versions.insert(LoginVersion::All);
                } else {
                    panic!("invalid value passed as login_logon_versions: '{w}'");
                }
            }
        } else if key == VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    self.insert_world_version(WorldVersion::Major(v), ty_name, file_info);
                    continue;
                } else if w == "*" {
                    self.insert_world_version(WorldVersion::All, ty_name, file_info);
                    continue;
                }

                let d: Vec<u8> = w.split('.').map(|a| a.parse::<u8>().unwrap()).collect();
                self.insert_world_version(
                    match d.as_slice() {
                        [major, minor] => WorldVersion::Minor(*major, *minor),
                        [major, minor, patch] => WorldVersion::Patch(*major, *minor, *patch),
                        [major, minor, patch, build] => {
                            WorldVersion::Exact(*major, *minor, *patch, u16::from(*build))
                        }
                        _ => panic!("incorrect world version string"),
                    },
                    ty_name,
                    file_info,
                );
            }
        } else if key == PASTE_VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    self.paste_versions.insert(WorldVersion::Major(v));
                    continue;
                } else if w == "*" {
                    panic!("Got all version for paste_versions, this is not valid, {self:#?}");
                }

                let d: Vec<u8> = w.split('.').map(|a| a.parse::<u8>().unwrap()).collect();
                self.paste_versions.insert(match d.as_slice() {
                    [major, minor] => WorldVersion::Minor(*major, *minor),
                    [major, minor, patch] => WorldVersion::Patch(*major, *minor, *patch),
                    [major, minor, patch, build] => {
                        WorldVersion::Exact(*major, *minor, *patch, u16::from(*build))
                    }
                    _ => panic!("incorrect world version string"),
                });
            }
        } else if key == COMPRESSED {
            self.compressed = Some(value.to_owned());
        } else if key == COMMENT {
            if let Some(comment) = &mut self.comment {
                comment.add(value);
            } else {
                let mut t = TagString::new();
                t.add(value);
                self.comment = Some(t);
            }
        } else if key == DISPLAY {
            self.display = Some(value.to_string());
        } else if key == TEST_STR {
            self.is_test.insert(value);
        } else if key == SKIP_STR {
            self.skip.insert(value);
        } else if key == UNIMPLEMENTED {
            self.unimplemented.insert(value);
        } else if key == RUST_BASE_TYPE {
            self.rust_base_ty.insert(value);
        } else if key == ZERO_IS_ALWAYS_VALID {
            self.zero_is_always_valid.insert(value);
        } else if key == NON_NETWORK_TYPE {
            self.non_network_type.insert(value);
        } else if key == USED_IN_UPDATE_MASK {
            self.used_in_update_mask.insert(value);
        } else if key == VALID_RANGE {
            let values = value.split(' ').collect::<Vec<_>>();
            self.valid_range = Some((values[0].parse().unwrap(), values[1].parse().unwrap()));
        } else if key == MAXIMUM_LENGTH {
            self.maximum_length = Some(value.parse().unwrap());
        } else {
            panic!("Unknown tag: '{key}'")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub(crate) struct BoolTag {
    inner: Option<bool>,
}

impl BoolTag {
    pub fn insert(&mut self, s: &str) {
        let value = if s == "true" {
            true
        } else if s == "false" {
            false
        } else {
            panic!("invalid value for tag: '{s}'");
        };

        if let Some(v) = self.inner {
            assert_eq!(
                v, value,
                "invalid overwrite for BoolTag, overwriting '{v}' with '{value}'",
            );
        } else {
            self.inner = Some(value);
        }
    }

    pub fn append(&mut self, other: Self) {
        if let Some(v) = self.inner {
            if let Some(value) = other.inner {
                assert_eq!(
                    v, value,
                    "invalid overwrite for BoolTag, overwriting '{v}' with '{value}'",
                );
            }
        } else if let Some(value) = other.inner {
            self.inner = Some(value);
        }
    }

    pub fn into_bool(self) -> bool {
        if let Some(v) = self.inner {
            v
        } else {
            false
        }
    }

    pub fn into_bool_with_default(self, default: bool) -> bool {
        if let Some(v) = self.inner {
            v
        } else {
            default
        }
    }
}
