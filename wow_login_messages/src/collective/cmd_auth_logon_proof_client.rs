use crate::collective::CollectiveMessage;
use crate::errors::CollectiveError;

type Main = crate::version_8::CMD_AUTH_LOGON_PROOF_Client;
type MainSecurityFlag = crate::version_8::CMD_AUTH_LOGON_PROOF_Client_SecurityFlag;
type MainSecurityFlagPin = crate::version_8::CMD_AUTH_LOGON_PROOF_Client_SecurityFlag_Pin;

type V2Main = crate::version_2::CMD_AUTH_LOGON_PROOF_Client;

type V3Main = crate::version_3::CMD_AUTH_LOGON_PROOF_Client;
type V3MainSecurityFlag = crate::version_3::CMD_AUTH_LOGON_PROOF_Client_SecurityFlag;

impl CollectiveMessage for Main {
    type Version2 = V2Main;
    type Version3 = V3Main;
    type Version5 = Self::Version3;
    type Version6 = Self::Version3;
    type Version7 = Self::Version3;
    type Version8 = Self;

    fn from_version_2(v: Self::Version2) -> Self {
        Self {
            client_public_key: v.client_public_key,
            client_proof: v.client_proof,
            crc_hash: v.crc_hash,
            telemetry_keys: v.telemetry_keys,
            security_flag: MainSecurityFlag::empty(),
        }
    }

    fn to_version_2(&self) -> Result<Self::Version2, CollectiveError> {
        Ok(V2Main {
            client_public_key: self.client_public_key,
            client_proof: self.client_proof,
            crc_hash: self.crc_hash,
            telemetry_keys: self.telemetry_keys.clone(),
        })
    }

    fn from_version_3(v: Self::Version3) -> Self {
        Self {
            client_public_key: v.client_public_key,
            client_proof: v.client_proof,
            crc_hash: v.crc_hash,
            telemetry_keys: v.telemetry_keys,
            security_flag: MainSecurityFlag::from_version_3(v.security_flag),
        }
    }

    fn to_version_3(&self) -> Result<Self::Version3, CollectiveError> {
        Ok(V3Main {
            client_public_key: self.client_public_key,
            client_proof: self.client_proof,
            crc_hash: self.crc_hash,
            telemetry_keys: self.telemetry_keys.clone(),
            security_flag: self.security_flag.to_version_3()?,
        })
    }

    fn from_version_5(v: Self::Version5) -> Self {
        Self::from_version_3(v)
    }

    fn to_version_5(&self) -> Result<Self::Version5, CollectiveError> {
        self.to_version_3()
    }

    fn from_version_6(v: Self::Version6) -> Self {
        Self::from_version_3(v)
    }

    fn to_version_6(&self) -> Result<Self::Version6, CollectiveError> {
        self.to_version_3()
    }

    fn from_version_7(v: Self::Version7) -> Self {
        Self::from_version_3(v)
    }

    fn to_version_7(&self) -> Result<Self::Version7, CollectiveError> {
        self.to_version_3()
    }
}

impl MainSecurityFlag {
    const fn to_version_3(&self) -> Result<V3MainSecurityFlag, CollectiveError> {
        Ok(if let Some(pin) = self.get_pin() {
            V3MainSecurityFlag::Pin {
                pin_hash: pin.pin_hash,
                pin_salt: pin.pin_salt,
            }
        } else {
            V3MainSecurityFlag::None
        })
    }

    const fn from_version_3(v: V3MainSecurityFlag) -> Self {
        match v {
            V3MainSecurityFlag::None => Self::empty(),
            V3MainSecurityFlag::Pin { pin_hash, pin_salt } => {
                Self::new_pin(MainSecurityFlagPin { pin_hash, pin_salt })
            }
        }
    }
}
