use core::cmp::{Ord, Ordering, PartialOrd};
use core::fmt::{Display, Formatter};
use core::option::Option;
use core::option::Option::Some;
use std::collections::BTreeSet;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum WorldVersion {
    Major(u8),
    Minor(u8, u8),
    Patch(u8, u8, u8),
    Exact(u8, u8, u8, u16),
    All,
}

impl PartialOrd<WorldVersion> for WorldVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WorldVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            WorldVersion::Major(m) => match other {
                WorldVersion::Major(om) => m.cmp(om),
                WorldVersion::Minor(om, _)
                | WorldVersion::Patch(om, _, _)
                | WorldVersion::Exact(om, _, _, _) => m.cmp(om).then(Ordering::Less),
                WorldVersion::All => Ordering::Greater,
            },
            WorldVersion::Minor(m, i) => match other {
                WorldVersion::Major(om) => m.cmp(om).then(Ordering::Greater),
                WorldVersion::Minor(om, oi) => m.cmp(om).then_with(|| i.cmp(oi)),
                WorldVersion::Patch(om, oi, _) | WorldVersion::Exact(om, oi, _, _) => {
                    m.cmp(om).then_with(|| i.cmp(oi)).then(Ordering::Less)
                }
                WorldVersion::All => Ordering::Greater,
            },
            WorldVersion::Patch(m, i, p) => match other {
                WorldVersion::Major(om) => m.cmp(om).then(Ordering::Greater),
                WorldVersion::Minor(om, oi) => {
                    m.cmp(om).then_with(|| i.cmp(oi)).then(Ordering::Greater)
                }
                WorldVersion::Patch(om, oi, op) => {
                    m.cmp(om).then_with(|| i.cmp(oi)).then_with(|| p.cmp(op))
                }
                WorldVersion::Exact(om, oi, op, _) => m
                    .cmp(om)
                    .then_with(|| i.cmp(oi))
                    .then_with(|| p.cmp(op))
                    .then(Ordering::Less),
                WorldVersion::All => Ordering::Greater,
            },
            WorldVersion::Exact(m, i, p, e) => match other {
                WorldVersion::Major(om) => m.cmp(om).then(Ordering::Greater),
                WorldVersion::Minor(om, oi) => {
                    m.cmp(om).then_with(|| i.cmp(oi)).then(Ordering::Greater)
                }
                WorldVersion::Patch(om, oi, op) => m
                    .cmp(om)
                    .then_with(|| i.cmp(oi))
                    .then_with(|| p.cmp(op))
                    .then(Ordering::Greater),
                WorldVersion::Exact(om, oi, op, oe) => m
                    .cmp(om)
                    .then_with(|| i.cmp(oi))
                    .then_with(|| p.cmp(op))
                    .then_with(|| e.cmp(oe)),
                WorldVersion::All => Ordering::Greater,
            },
            WorldVersion::All => match other {
                WorldVersion::All => Ordering::Equal,
                WorldVersion::Major(_)
                | WorldVersion::Minor(_, _)
                | WorldVersion::Patch(_, _, _)
                | WorldVersion::Exact(_, _, _, _) => Ordering::Less,
            },
        }
    }
}

#[test]
fn world_version_ordering() {
    let mut v = [
        WorldVersion::Exact(1, 12, 1, 0),
        WorldVersion::Patch(3, 3, 5),
        WorldVersion::Major(1),
        WorldVersion::Minor(2, 4),
        WorldVersion::Major(3),
    ];

    v.sort();

    let expected = [
        WorldVersion::Major(1),
        WorldVersion::Exact(1, 12, 1, 0),
        WorldVersion::Minor(2, 4),
        WorldVersion::Major(3),
        WorldVersion::Patch(3, 3, 5),
    ];

    assert_eq!(v, expected);
}

impl WorldVersion {
    /// self has some overlap with other
    pub fn overlaps(&self, other: &Self) -> bool {
        match self {
            WorldVersion::Major(m) => match other {
                WorldVersion::Major(om)
                | WorldVersion::Minor(om, _)
                | WorldVersion::Patch(om, _, _)
                | WorldVersion::Exact(om, _, _, _) => m == om,
                WorldVersion::All => true,
            },
            WorldVersion::Minor(m, i) => match other {
                WorldVersion::Major(om) => om == m,
                WorldVersion::Minor(om, oi) => om == m && oi == i,
                WorldVersion::Patch(om, oi, _) => om == m && oi == i,
                WorldVersion::Exact(om, oi, _, _) => om == m && oi == i,
                WorldVersion::All => true,
            },
            WorldVersion::Patch(m, i, p) => match other {
                WorldVersion::Major(om) => om == m,
                WorldVersion::Minor(om, oi) => om == m && oi == i,
                WorldVersion::Patch(om, oi, op) => om == m && oi == i && op == p,
                WorldVersion::Exact(om, oi, op, _) => om == m && oi == i && op == p,
                WorldVersion::All => true,
            },
            WorldVersion::Exact(m, i, p, e) => match other {
                WorldVersion::Major(om) => om == m,
                WorldVersion::Minor(om, oi) => om == m && oi == i,
                WorldVersion::Patch(om, oi, op) => om == m && oi == i && op == p,
                WorldVersion::Exact(om, oi, op, oe) => om == m && oi == i && op == p && oe == e,
                WorldVersion::All => true,
            },
            WorldVersion::All => true,
        }
    }

    /// self completely fulfills other
    pub fn covers(&self, other: &Self) -> bool {
        match other {
            WorldVersion::Major(om) => match self {
                WorldVersion::Major(m) => m == om,
                WorldVersion::Minor(_, _) => false,
                WorldVersion::Patch(_, _, _) => false,
                WorldVersion::Exact(_, _, _, _) => false,
                WorldVersion::All => true,
            },
            WorldVersion::Minor(om, oi) => match self {
                WorldVersion::Major(m) => m == om,
                WorldVersion::Minor(m, i) => m == om && i == oi,
                WorldVersion::Patch(_, _, _) => false,
                WorldVersion::Exact(_, _, _, _) => false,
                WorldVersion::All => true,
            },
            WorldVersion::Patch(om, oi, op) => match self {
                WorldVersion::Major(m) => m == om,
                WorldVersion::Minor(m, i) => m == om && i == oi,
                WorldVersion::Patch(m, i, p) => m == om && i == oi && p == op,
                WorldVersion::Exact(_, _, _, _) => false,
                WorldVersion::All => true,
            },
            WorldVersion::Exact(om, oi, op, oe) => match self {
                WorldVersion::Major(m) => m == om,
                WorldVersion::Minor(m, i) => m == om && i == oi,
                WorldVersion::Patch(m, i, p) => m == om && i == oi && p == op,
                WorldVersion::Exact(m, i, p, e) => m == om && i == oi && p == op && e == oe,
                WorldVersion::All => true,
            },
            WorldVersion::All => matches!(self, WorldVersion::All),
        }
    }

    pub fn as_module_case(&self) -> String {
        self.to_string().replace('.', "_")
    }

    pub fn is_main_version(&self) -> bool {
        self.try_as_major_world().is_some()
    }

    pub(crate) fn try_as_major_world(&self) -> Option<MajorWorldVersion> {
        const VANILLA: WorldVersion = WorldVersion::Minor(1, 12);
        const TBC: WorldVersion = WorldVersion::Exact(2, 4, 3, 8606);
        const WRATH: WorldVersion = WorldVersion::Exact(3, 3, 5, 12340);

        if self.covers(&VANILLA) {
            Some(MajorWorldVersion::Vanilla)
        } else if self.covers(&TBC) {
            Some(MajorWorldVersion::BurningCrusade)
        } else if self.covers(&WRATH) {
            Some(MajorWorldVersion::Wrath)
        } else {
            None
        }
    }

    pub(crate) fn as_major_world(&self) -> MajorWorldVersion {
        self.try_as_major_world().unwrap()
    }
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

impl LoginVersion {
    pub(crate) fn as_module_case(&self) -> String {
        match self {
            LoginVersion::Specific(v) => format!("version_{}", v),
            LoginVersion::All => "all".to_string(),
        }
    }

    /// Self overlaps with other
    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        match self {
            LoginVersion::Specific(e) => match other {
                LoginVersion::Specific(oe) => e == oe,
                LoginVersion::All => true,
            },
            LoginVersion::All => true,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Version {
    Login(LoginVersion),
    World(WorldVersion),
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Version::Login(l) => match other {
                Version::Login(ol) => l.cmp(ol),
                Version::World(_) => Ordering::Less,
            },
            Version::World(w) => match other {
                Version::Login(_) => Ordering::Greater,
                Version::World(ow) => w.cmp(ow),
            },
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Version {
    pub(crate) fn is_world(&self) -> bool {
        match self {
            Version::Login(_) => false,
            Version::World(_) => true,
        }
    }

    pub(crate) fn as_world(&self) -> WorldVersion {
        match self {
            Version::Login(_) => unreachable!(),
            Version::World(w) => *w,
        }
    }

    pub(crate) fn as_major_world(&self) -> MajorWorldVersion {
        self.as_world().as_major_world()
    }

    pub(crate) fn to_module_case(self) -> String {
        match self {
            Version::Login(l) => l.as_module_case(),
            Version::World(l) => l.as_module_case(),
        }
    }
}

impl From<LoginVersion> for Version {
    fn from(l: LoginVersion) -> Self {
        Self::Login(l)
    }
}

impl From<WorldVersion> for Version {
    fn from(l: WorldVersion) -> Self {
        Self::World(l)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum MajorWorldVersion {
    Vanilla,
    BurningCrusade,
    Wrath,
}

impl MajorWorldVersion {
    pub(crate) fn encryption_path(&self) -> &'static str {
        match self {
            MajorWorldVersion::Vanilla => "wow_srp::vanilla_header",
            MajorWorldVersion::BurningCrusade => "wow_srp::tbc_header",
            MajorWorldVersion::Wrath => "wow_srp::wrath_header",
        }
    }

    pub(crate) fn module_name(&self) -> &'static str {
        self.feature_name()
    }

    pub(crate) fn feature_name(&self) -> &'static str {
        match self {
            MajorWorldVersion::Vanilla => "vanilla",
            MajorWorldVersion::BurningCrusade => "tbc",
            MajorWorldVersion::Wrath => "wrath",
        }
    }

    pub(crate) fn wrath_or_greater(&self) -> bool {
        match self {
            MajorWorldVersion::Vanilla | MajorWorldVersion::BurningCrusade => false,
            MajorWorldVersion::Wrath => true,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum AllVersions {
    Login(BTreeSet<LoginVersion>),
    World(BTreeSet<WorldVersion>),
}
