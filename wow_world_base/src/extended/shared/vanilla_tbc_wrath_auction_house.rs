use crate::shared::auction_house_vanilla_tbc_wrath::AuctionHouse;

impl AuctionHouse {
    /// Returns the percentage for depositing items for 12 hours.
    ///
    /// 24 hours would be double this value, and 48 hours would be four times this value.
    ///
    /// The value is returned as a whole number so a value of `5_u8` would mean `5%`.
    ///
    /// Goblin auction houses have a 25% deposit and all others have a 5%.
    pub const fn deposit_percentage(&self) -> u8 {
        match self {
            AuctionHouse::Goblin => 25,
            _ => 5,
        }
    }

    /// Returns the percentage of auction value the house takes.
    ///
    /// The value is returned as a whole number so a value of `5_u8` would mean `5%`.
    ///
    /// Goblin auction houses have a 15% cut and all others have a 5%.
    pub const fn cut_percentage(&self) -> u8 {
        match self {
            AuctionHouse::Goblin => 15,
            _ => 5,
        }
    }
}

impl AuctionHouse {
    /// Returns the faction the auction house belongs to.
    #[cfg(feature = "vanilla")]
    pub const fn vanilla_faction(&self) -> crate::vanilla::Faction {
        use crate::vanilla::Faction;

        match self {
            AuctionHouse::Stormwind => Faction::PlayerHuman,
            AuctionHouse::Alliance => Faction::PlayerDwarf,
            AuctionHouse::Darnassus => Faction::PlayerNightElf,
            AuctionHouse::Undercity => Faction::PlayerUndead,
            AuctionHouse::ThunderBluff => Faction::PlayerTauren,
            AuctionHouse::Horde => Faction::PlayerOrc,
            AuctionHouse::Goblin => Faction::Gadgetzan,
        }
    }

    /// Returns the faction the auction house belongs to.
    #[cfg(feature = "tbc")]
    pub const fn tbc_faction(&self) -> crate::tbc::Faction {
        use crate::tbc::Faction;

        match self {
            AuctionHouse::Stormwind => Faction::PlayerHuman,
            AuctionHouse::Alliance => Faction::PlayerDwarf,
            AuctionHouse::Darnassus => Faction::PlayerNightElf,
            AuctionHouse::Undercity => Faction::PlayerUndead,
            AuctionHouse::ThunderBluff => Faction::PlayerTauren,
            AuctionHouse::Horde => Faction::PlayerOrc,
            AuctionHouse::Goblin => Faction::Gadgetzan,
        }
    }

    /// Returns the faction the auction house belongs to.
    #[cfg(feature = "wrath")]
    pub const fn wrath_faction(&self) -> crate::wrath::Faction {
        use crate::wrath::Faction;

        match self {
            AuctionHouse::Stormwind => Faction::PlayerHuman,
            AuctionHouse::Alliance => Faction::PlayerDwarf,
            AuctionHouse::Darnassus => Faction::PlayerNightElf,
            AuctionHouse::Undercity => Faction::PlayerUndead,
            AuctionHouse::ThunderBluff => Faction::PlayerTauren,
            AuctionHouse::Horde => Faction::PlayerOrc,
            AuctionHouse::Goblin => Faction::Gadgetzan,
        }
    }
}
