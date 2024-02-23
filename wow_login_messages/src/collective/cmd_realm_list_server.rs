use crate::collective::CollectiveMessage;
use crate::errors::CollectiveError;

type Main = crate::version_8::CMD_REALM_LIST_Server;
type MainRealm = crate::version_8::Realm;
type MainRealmFlag = crate::version_8::Realm_RealmFlag;

type V2Main = crate::version_2::CMD_REALM_LIST_Server;
type V2MainRealm = crate::version_2::Realm;
type V2MainRealmFlag = crate::version_2::RealmFlag;

type V5Main = crate::version_5::CMD_REALM_LIST_Server;
type V5MainRealm = crate::version_5::Realm;
type V5MainRealmFlag = crate::version_5::RealmFlag;

impl CollectiveMessage for Main {
    type Version2 = V2Main;
    type Version3 = Self::Version2;
    type Version5 = V5Main;
    type Version6 = Self::Version5;
    type Version7 = Self::Version5;
    type Version8 = Self;

    fn from_version_2(v: Self::Version2) -> Self {
        Self {
            realms: v
                .realms
                .into_iter()
                .map(MainRealm::from_version_2)
                .collect(),
        }
    }

    fn to_version_2(&self) -> Result<Self::Version2, CollectiveError> {
        Ok(Self::Version2 {
            realms: self.realms.iter().map(|a| a.to_version_2()).collect(),
        })
    }

    fn from_version_3(v: Self::Version3) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_3(&self) -> Result<Self::Version3, CollectiveError> {
        self.to_version_2()
    }

    fn from_version_5(v: Self::Version5) -> Self {
        Self {
            realms: v
                .realms
                .into_iter()
                .map(MainRealm::from_version_5)
                .collect(),
        }
    }

    fn to_version_5(&self) -> Result<Self::Version5, CollectiveError> {
        Ok(Self::Version5 {
            realms: self.realms.iter().map(|a| a.to_version_5()).collect(),
        })
    }

    fn from_version_6(v: Self::Version6) -> Self {
        Self::from_version_5(v)
    }

    fn to_version_6(&self) -> Result<Self::Version6, CollectiveError> {
        self.to_version_5()
    }

    fn from_version_7(v: Self::Version7) -> Self {
        Self::from_version_5(v)
    }

    fn to_version_7(&self) -> Result<Self::Version7, CollectiveError> {
        self.to_version_5()
    }
}

impl MainRealm {
    fn from_version_2(v: V2MainRealm) -> Self {
        Self {
            realm_type: v.realm_type,
            locked: false,
            flag: MainRealmFlag::from_version_2(&v.flag),
            name: v.name,
            address: v.address,
            population: v.population,
            number_of_characters_on_realm: v.number_of_characters_on_realm,
            category: v.category,
            realm_id: v.realm_id,
        }
    }

    fn to_version_2(&self) -> V2MainRealm {
        V2MainRealm {
            realm_type: self.realm_type,
            flag: self.flag.to_version_2(),
            name: self.name.clone(),
            address: self.address.clone(),
            population: self.population,
            number_of_characters_on_realm: self.number_of_characters_on_realm,
            category: self.category,
            realm_id: self.realm_id,
        }
    }

    fn from_version_5(v: V5MainRealm) -> Self {
        Self {
            realm_type: v.realm_type,
            locked: v.locked,
            flag: MainRealmFlag::from_version_5(&v.flag),
            name: v.name,
            address: v.address,
            population: v.population,
            number_of_characters_on_realm: v.number_of_characters_on_realm,
            category: v.category,
            realm_id: v.realm_id,
        }
    }

    fn to_version_5(&self) -> V5MainRealm {
        V5MainRealm {
            realm_type: self.realm_type,
            locked: self.locked,
            flag: self.flag.to_version_5(),
            name: self.name.clone(),
            address: self.address.clone(),
            population: self.population,
            number_of_characters_on_realm: self.number_of_characters_on_realm,
            category: self.category,
            realm_id: self.realm_id,
        }
    }
}

impl MainRealmFlag {
    const fn from_version_2(v: &V2MainRealmFlag) -> Self {
        Self::new(v.as_int(), None)
    }

    #[allow(clippy::wrong_self_convention)]
    const fn to_version_2(&self) -> V2MainRealmFlag {
        V2MainRealmFlag::new(self.as_int())
    }

    const fn from_version_5(v: &V5MainRealmFlag) -> Self {
        Self::new(v.as_int(), None)
    }

    #[allow(clippy::wrong_self_convention)]
    const fn to_version_5(&self) -> V5MainRealmFlag {
        V5MainRealmFlag::new(self.as_int())
    }
}
