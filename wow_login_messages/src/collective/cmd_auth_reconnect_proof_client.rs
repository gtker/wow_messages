use crate::collective::CollectiveMessage;
use crate::errors::CollectiveError;

type Main = crate::version_2::cmd_auth_reconnect_proof_client::CMD_AUTH_RECONNECT_PROOF_Client;

impl CollectiveMessage for Main {
    type Version2 = Main;
    type Version3 = Self::Version2;
    type Version5 = Self::Version2;
    type Version6 = Self::Version2;
    type Version7 = Self::Version2;
    type Version8 = Self::Version2;

    fn from_version_2(v: Self::Version2) -> Self {
        v
    }

    fn to_version_2(&self) -> Result<Self::Version2, CollectiveError> {
        Ok(*self)
    }

    fn from_version_3(v: Self::Version3) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_3(&self) -> Result<Self::Version3, CollectiveError> {
        Ok(*self)
    }

    fn from_version_5(v: Self::Version5) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_5(&self) -> Result<Self::Version5, CollectiveError> {
        Ok(*self)
    }

    fn from_version_6(v: Self::Version6) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_6(&self) -> Result<Self::Version6, CollectiveError> {
        Ok(*self)
    }

    fn from_version_7(v: Self::Version7) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_7(&self) -> Result<Self::Version7, CollectiveError> {
        Ok(*self)
    }
}
