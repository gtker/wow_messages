use crate::collective::CollectiveMessage;
use crate::version_8::CMD_XFER_INITIATE;

impl CollectiveMessage for CMD_XFER_INITIATE {
    type Version2 = Self;
    type Version3 = Self;
    type Version5 = Self;
    type Version6 = Self;
    type Version7 = Self;
    type Version8 = Self;

    fn from_version_2(v: Self::Version2) -> Self {
        v
    }

    fn to_version_2(&self) -> Self::Version2 {
        self.clone()
    }

    fn from_version_3(v: Self::Version3) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_3(&self) -> Self::Version3 {
        self.clone()
    }

    fn from_version_5(v: Self::Version5) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_5(&self) -> Self::Version5 {
        self.clone()
    }

    fn from_version_6(v: Self::Version6) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_6(&self) -> Self::Version6 {
        self.clone()
    }

    fn from_version_7(v: Self::Version7) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_7(&self) -> Self::Version7 {
        self.clone()
    }
}
