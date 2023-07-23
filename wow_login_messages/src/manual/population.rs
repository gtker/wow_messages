#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Population {
    GreenRecommended,
    RedFull,
    BlueRecommended,
    Other(f32),
}

impl Population {
    pub(crate) const fn as_int(&self) -> f32 {
        match self {
            Self::GreenRecommended => 200.0,
            Self::RedFull => 400.0,
            Self::BlueRecommended => 600.0,
            Self::Other(v) => *v,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [Self::GreenRecommended, Self::RedFull, Self::BlueRecommended]
    }
}

impl Default for Population {
    fn default() -> Self {
        Self::GreenRecommended
    }
}

impl std::fmt::Display for Population {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GreenRecommended => f.write_str("GreenRecommended"),
            Self::RedFull => f.write_str("RedFull"),
            Self::BlueRecommended => f.write_str("BlueRecommended"),
            Self::Other(v) => f.write_fmt(format_args!("Other({v})")),
        }
    }
}

impl From<f32> for Population {
    fn from(value: f32) -> Self {
        if value == 200.0 {
            Self::GreenRecommended
        } else if value == 400.0 {
            Self::RedFull
        } else if value == 600.0 {
            Self::BlueRecommended
        } else {
            Self::Other(value)
        }
    }
}
