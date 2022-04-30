use std::fmt::{Display, Formatter};

use crate::{COMMENT, DESCRIPTION, DISPLAY, LOGIN_LOGON_VERSIONS, TEST_STR, VERSIONS};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum WorldVersion {
    Major(u8),
    Minor(u8, u8),
    Patch(u8, u8, u8),
    Exact(u8, u8, u8, u8),
    All,
}

impl Display for WorldVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldVersion::Major(m) => f.write_fmt(format_args!("{}", m)),
            WorldVersion::Minor(m, i) => f.write_fmt(format_args!("{}.{}", m, i)),
            WorldVersion::Patch(m, i, p) => f.write_fmt(format_args!("{}.{}.{}", m, i, p)),
            WorldVersion::Exact(m, i, p, b) => f.write_fmt(format_args!("{}.{}.{}.{}", m, i, p, b)),
            WorldVersion::All => f.write_str("*"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum LoginVersion {
    Specific(u8),
    All,
}

impl Display for LoginVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginVersion::Specific(v) => f.write_fmt(format_args!("{}", v)),
            LoginVersion::All => f.write_str("*"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tags {
    inner: Vec<Tag>,
    login_logon_versions: Vec<LoginVersion>,
    world_versions: Vec<WorldVersion>,
    description: String,
    comment: String,
    display: String,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            inner: vec![],
            login_logon_versions: vec![],
            world_versions: vec![],
            description: "".to_string(),
            comment: "".to_string(),
            display: "".to_string(),
        }
    }

    pub fn push(&mut self, t: Tag) {
        self.append_or_insert(t.key(), t.value());
    }

    pub fn append(&mut self, t: &Tags) {
        for tag in &t.inner {
            self.append_or_insert(tag.key(), tag.value());
        }
    }

    pub fn append_or_insert(&mut self, key: &str, value: &str) {
        if key == LOGIN_LOGON_VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    self.login_logon_versions.push(LoginVersion::Specific(v));
                } else if w == "*" {
                    self.login_logon_versions.push(LoginVersion::All);
                } else {
                    panic!("invalid value passed as login_logon_versions: '{}'", w);
                }
            }
            self.login_logon_versions.sort();
            self.login_logon_versions.dedup();
            if self.login_logon_versions.contains(&LoginVersion::All) {
                self.login_logon_versions = vec![LoginVersion::All];
            }

            //return;
        } else if key == VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    self.world_versions.push(WorldVersion::Major(v));
                    continue;
                } else if w == "*" {
                    self.world_versions.push(WorldVersion::All);
                    continue;
                }

                let d: Vec<u8> = w.split('.').map(|a| a.parse::<u8>().unwrap()).collect();
                self.world_versions.push(match d.len() {
                    2 => WorldVersion::Minor(d[0], d[1]),
                    3 => WorldVersion::Patch(d[0], d[1], d[2]),
                    4 => WorldVersion::Exact(d[0], d[1], d[2], d[3]),
                    _ => panic!("incorrect world version string"),
                });
            }
            self.world_versions.sort();
            self.world_versions.dedup();
            if self.world_versions.contains(&WorldVersion::All) {
                self.world_versions = vec![WorldVersion::All];
            }
        } else if key == DESCRIPTION {
            if self.description == "" {
                self.description = value.to_string();
            } else {
                self.description.push_str("\n");
                self.description.push_str(value);
            }
        } else if key == COMMENT {
            if self.comment == "" {
                self.comment = value.to_string();
            } else {
                self.comment.push_str("\n");
                self.comment.push_str(value);
            }
        } else if key == DISPLAY {
            self.display = value.to_string();
        }

        for v in self.inner.iter_mut() {
            if v.key == key {
                v.value += " ";
                v.value += value;
                return;
            }
        }

        self.inner.push(Tag::new(key, value));
    }

    pub fn contains(&self, name: &str) -> bool {
        self.inner.iter().any(|a| a.key == name)
    }

    pub fn get_ref(&self, name: &str) -> Option<&str> {
        match self.inner.iter().find(|a| a.key == name) {
            None => None,
            Some(v) => Some(v.value()),
        }
    }

    pub fn has_all_versions(&self, tags: &Tags) -> bool {
        // if self has all versions of tags
        if tags.contains(TEST_STR) && self.contains(TEST_STR) {
            return true;
        }

        for t in tags.logon_versions() {
            match self.logon_versions().contains(t)
                || self.logon_versions().contains(&LoginVersion::All)
            {
                true => {}
                false => {
                    return false;
                }
            }
        }

        for t in tags.versions() {
            match self.versions().contains(t) || self.versions().contains(&WorldVersion::All) {
                true => {}
                false => {
                    return false;
                }
            }
        }

        true
    }

    pub fn has_version_intersections(&self, tags: &Tags) -> bool {
        if tags.contains(TEST_STR) && self.contains(TEST_STR) {
            return true;
        }

        for t in &self.login_logon_versions {
            match tags.login_logon_versions.contains(t)
                || self.login_logon_versions.contains(&LoginVersion::All)
            {
                true => return true,
                false => {}
            }
        }

        for t in self.versions() {
            match tags.versions().contains(t) || self.versions().contains(&WorldVersion::All) {
                true => return true,
                false => {}
            }
        }

        false
    }

    pub fn has_version_all(&self) -> bool {
        self.login_logon_versions.contains(&LoginVersion::All)
            || self.world_versions.contains(&WorldVersion::All)
    }

    pub fn has_wildcard_logon_version(&self) -> bool {
        self.logon_versions().contains(&LoginVersion::All)
    }

    pub fn logon_versions(&self) -> &[LoginVersion] {
        &self.login_logon_versions
    }

    pub fn versions(&self) -> &[WorldVersion] {
        &self.world_versions
    }

    pub fn has_login_version(&self) -> bool {
        if !self.login_logon_versions.is_empty() {
            assert!(self.world_versions.is_empty());
            return true;
        } else if !self.world_versions.is_empty() {
            assert!(self.login_logon_versions.is_empty());
            return false;
        }

        false
    }

    pub fn description(&self) -> Option<&str> {
        if self.description == "" {
            None
        } else {
            Some(&self.description)
        }
    }

    pub fn comment(&self) -> Option<&str> {
        if self.comment == "" {
            None
        } else {
            Some(&self.comment)
        }
    }

    pub fn display(&self) -> Option<&str> {
        if self.display == "" {
            None
        } else {
            Some(&self.display)
        }
    }

    pub fn has_display(&self) -> bool {
        self.display != ""
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tag {
    key: String,
    value: String,
}

impl Tag {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
