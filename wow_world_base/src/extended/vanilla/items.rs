use crate::extended::vanilla::vanilla_race_class_match;
use crate::vanilla::{ItemSlot, RaceClass};

vanilla_race_class_match!(starter_items, &'static [StarterItem], RaceClass);

pub struct StarterItem {
    pub item: u32,
    pub ty: ItemSlot,
    pub amount: u8,
}

impl StarterItem {
    pub(crate) const fn multi(item: u32, ty: ItemSlot, amount: u8) -> Self {
        Self { item, ty, amount }
    }

    pub(crate) const fn single(item: u32, ty: ItemSlot) -> Self {
        Self {
            item,
            ty,
            amount: 1,
        }
    }
}

const HUMAN_WARRIOR: &[StarterItem] = &[
    StarterItem::single(38, ItemSlot::Shirt), // Recruit's Shirt
    StarterItem::single(39, ItemSlot::Legs),  // Recruit's Pants
    StarterItem::single(40, ItemSlot::Boots), // Recruit's Boots
    StarterItem::single(25, ItemSlot::MainHand), // Worn Shortsword
    StarterItem::single(2362, ItemSlot::OffHand), // Worn Wooden Shield
    StarterItem::multi(117, ItemSlot::Inventory0, 5), // Tough Jerky
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const HUMAN_PALADIN: &[StarterItem] = &[
    StarterItem::single(45, ItemSlot::Shirt), // Squire's Shirt
    StarterItem::single(43, ItemSlot::Boots), // Squire's Boots
    StarterItem::single(44, ItemSlot::Legs),  // Squire's Pants
    StarterItem::single(6948, ItemSlot::Inventory0), // Hearthstone
    StarterItem::single(2361, ItemSlot::MainHand), // Battleworn Hammer
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::multi(2070, ItemSlot::Inventory2, 5), // Darnassian Bleu
];
const HUMAN_ROGUE: &[StarterItem] = &[
    StarterItem::single(49, ItemSlot::Shirt), // Footpad's Shirt
    StarterItem::single(47, ItemSlot::Boots), // Footpad's Shoes
    StarterItem::single(48, ItemSlot::Legs),  // Footpad's Pants
    StarterItem::multi(2947, ItemSlot::RangedOrRelic, 200), // Small Throwing Knife
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::multi(2070, ItemSlot::Inventory0, 5), // Darnassian Bleu
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const HUMAN_PRIEST: &[StarterItem] = &[
    StarterItem::single(6098, ItemSlot::Chest), // Neophyte's Robe
    StarterItem::single(52, ItemSlot::Legs),    // Neophyte's Pants
    StarterItem::single(53, ItemSlot::Shirt),   // Neophyte's Shirt
    StarterItem::single(51, ItemSlot::Boots),   // Neophyte's Boots
    StarterItem::single(36, ItemSlot::MainHand), // Worn Mace
    StarterItem::multi(159, ItemSlot::Inventory0, 5), // Refreshing Spring Water
    StarterItem::multi(2070, ItemSlot::Inventory1, 5), // Darnassian Bleu
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const HUMAN_MAGE: &[StarterItem] = &[
    StarterItem::single(56, ItemSlot::Chest), // Apprentice's Robe
    StarterItem::single(1395, ItemSlot::Legs), // Apprentice's Pants
    StarterItem::single(55, ItemSlot::Boots), // Apprentice's Boots
    StarterItem::single(35, ItemSlot::MainHand), // Bent Staff
    StarterItem::single(6096, ItemSlot::Shirt), // Apprentice's Shirt
    StarterItem::multi(2070, ItemSlot::Inventory0, 5), // Darnassian Bleu
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const HUMAN_WARLOCK: &[StarterItem] = &[
    StarterItem::single(57, ItemSlot::Chest),   // Acolyte's Robe
    StarterItem::single(6097, ItemSlot::Shirt), // Acolyte's Shirt
    StarterItem::single(1396, ItemSlot::Legs),  // Acolyte's Pants
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::single(59, ItemSlot::Boots),   // Acolyte's Shoes
    StarterItem::multi(4604, ItemSlot::Inventory0, 5), // Forest Mushroom Cap
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const ORC_WARRIOR: &[StarterItem] = &[
    StarterItem::single(6125, ItemSlot::Shirt), // Brawler's Harness
    StarterItem::single(139, ItemSlot::Legs),   // Brawler's Pants
    StarterItem::single(140, ItemSlot::Boots),  // Brawler's Boots
    StarterItem::single(6948, ItemSlot::Inventory0), // Hearthstone
    StarterItem::single(12282, ItemSlot::MainHand), // Worn Battleaxe
    StarterItem::multi(117, ItemSlot::Inventory1, 5), // Tough Jerky
];
const ORC_HUNTER: &[StarterItem] = &[
    StarterItem::single(127, ItemSlot::Shirt), // Trapper's Shirt
    StarterItem::single(6126, ItemSlot::Legs), // Trapper's Pants
    StarterItem::single(6127, ItemSlot::Boots), // Trapper's Boots
    StarterItem::multi(159, ItemSlot::Inventory0, 5), // Refreshing Spring Water
    StarterItem::single(37, ItemSlot::MainHand), // Worn Axe
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
    StarterItem::single(2101, ItemSlot::Bag1), // Light Quiver
    StarterItem::single(2504, ItemSlot::RangedOrRelic), // Worn Shortbow
    StarterItem::multi(117, ItemSlot::Inventory2, 5), // Tough Jerky
    StarterItem::multi(2512, ItemSlot::Inventory3, 200), // Rough Arrow
];
const ORC_ROGUE: &[StarterItem] = &[
    StarterItem::single(2105, ItemSlot::Shirt), // Thug Shirt
    StarterItem::single(120, ItemSlot::Legs),   // Thug Pants
    StarterItem::single(121, ItemSlot::Boots),  // Thug Boots
    StarterItem::multi(3111, ItemSlot::RangedOrRelic, 200), // Crude Throwing Axe
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::multi(117, ItemSlot::Inventory0, 5), // Tough Jerky
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const ORC_SHAMAN: &[StarterItem] = &[
    StarterItem::single(154, ItemSlot::Shirt), // Primitive Mantle
    StarterItem::single(153, ItemSlot::Legs),  // Primitive Kilt
    StarterItem::single(6948, ItemSlot::Inventory0), // Hearthstone
    StarterItem::single(36, ItemSlot::MainHand), // Worn Mace
    StarterItem::multi(117, ItemSlot::Inventory1, 5), // Tough Jerky
    StarterItem::multi(159, ItemSlot::Inventory2, 5), // Refreshing Spring Water
];
const ORC_WARLOCK: &[StarterItem] = &[
    StarterItem::single(6129, ItemSlot::Chest), // Acolyte's Robe
    StarterItem::single(1396, ItemSlot::Legs),  // Robe's Pants
    StarterItem::single(59, ItemSlot::Boots),   // Acolyte's Shoes
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::single(6948, ItemSlot::Inventory0), // Hearthstone
    StarterItem::multi(117, ItemSlot::Inventory1, 5), // Tough Jerky
    StarterItem::multi(159, ItemSlot::Inventory2, 5), // Refreshing Spring Water
];
const DWARF_WARRIOR: &[StarterItem] = &[
    StarterItem::single(38, ItemSlot::Shirt), // Recruit's Shirt
    StarterItem::single(39, ItemSlot::Legs),  // Recruit's Pants
    StarterItem::single(40, ItemSlot::Boots), // Recruit's Boots
    StarterItem::single(12282, ItemSlot::MainHand), // Worn Battleaxe
    StarterItem::single(6948, ItemSlot::Inventory0), // Hearthstone
    StarterItem::multi(117, ItemSlot::Inventory1, 5), // Tough Jerky
];
const DWARF_PALADIN: &[StarterItem] = &[
    StarterItem::single(6117, ItemSlot::Shirt), // Squire's Shirt
    StarterItem::single(6118, ItemSlot::Legs),  // Squire's Pants
    StarterItem::single(43, ItemSlot::Boots),   // Squire's Boots
    StarterItem::single(2361, ItemSlot::MainHand), // Battleworn Hammer
    StarterItem::multi(4540, ItemSlot::Inventory0, 5), // Tough Hunk of Bread
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const DWARF_HUNTER: &[StarterItem] = &[
    StarterItem::single(148, ItemSlot::Shirt), // Rugged Trapper's Shirt
    StarterItem::single(147, ItemSlot::Legs),  // Rugged Trapper's Pants
    StarterItem::single(129, ItemSlot::Boots), // Rugged Trapper's Boots
    StarterItem::single(37, ItemSlot::MainHand), // Worn Axe
    StarterItem::multi(159, ItemSlot::Inventory0, 5), // Refreshing Spring Water
    StarterItem::single(2102, ItemSlot::Bag1), // Small Ammo Pouch
    StarterItem::single(2508, ItemSlot::RangedOrRelic), // Old Blunderbuss
    StarterItem::multi(2516, ItemSlot::Inventory3, 200), // Light Shot
    StarterItem::multi(117, ItemSlot::Inventory1, 5), // Tough Jerky
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const DWARF_ROGUE: &[StarterItem] = &[
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::single(49, ItemSlot::Shirt),      // Footpad's Shirt
    StarterItem::single(48, ItemSlot::Legs),       // Footpad's Pants
    StarterItem::single(47, ItemSlot::Boots),      // Footpad's Shoes
    StarterItem::multi(4540, ItemSlot::Inventory0, 5), // Tough Hunk of Bread
    StarterItem::multi(3111, ItemSlot::RangedOrRelic, 200), // Crude Throwing Axe
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const DWARF_PRIEST: &[StarterItem] = &[
    StarterItem::single(6098, ItemSlot::Chest), // Neophyte's Robe
    StarterItem::single(53, ItemSlot::Shirt),   // Neophyte's Shirt
    StarterItem::single(52, ItemSlot::Legs),    // Neophyte's Pants
    StarterItem::single(51, ItemSlot::Boots),   // Neophyte's Boots
    StarterItem::single(36, ItemSlot::MainHand), // Worn Mace
    StarterItem::multi(159, ItemSlot::Inventory0, 5), // Refreshing Spring Water
    StarterItem::multi(4540, ItemSlot::Inventory1, 5), // Tough Hunk of Bread
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const NIGHT_ELF_WARRIOR: &[StarterItem] = &[
    StarterItem::single(25, ItemSlot::MainHand), // Worn Shortsword
    StarterItem::single(6120, ItemSlot::Shirt),  // Recruit's Shirt
    StarterItem::single(6121, ItemSlot::Legs),   // Recruit's Pants
    StarterItem::single(6122, ItemSlot::Boots),  // Recruit's Boots
    StarterItem::single(2362, ItemSlot::OffHand), // Worn Wooden Shield
    StarterItem::multi(117, ItemSlot::Inventory0, 5), // Tough Jerky
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const NIGHT_ELF_HUNTER: &[StarterItem] = &[
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::single(148, ItemSlot::Shirt),     // Rugged Trapper's Shirt
    StarterItem::single(147, ItemSlot::Legs),      // Rugged Trapper's Pants
    StarterItem::single(129, ItemSlot::Boots),     // Rugged Trapper's Boots
    StarterItem::multi(159, ItemSlot::Inventory0, 5), // Refreshing Spring Water
    StarterItem::single(2504, ItemSlot::RangedOrRelic), // Worn Shortbow
    StarterItem::single(2101, ItemSlot::Bag1),     // Light Quiver
    StarterItem::multi(2512, ItemSlot::Inventory3, 200), // Rough Arrow
    StarterItem::multi(117, ItemSlot::Inventory1, 5), // Tough Jerky
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const NIGHT_ELF_ROGUE: &[StarterItem] = &[
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::single(49, ItemSlot::Shirt),      // Footpad's Shirt
    StarterItem::single(48, ItemSlot::Legs),       // Footpad's Pants
    StarterItem::single(47, ItemSlot::Boots),      // Footpad's Shoes
    StarterItem::multi(4540, ItemSlot::Inventory0, 5), // Tough Hunk of Bread
    StarterItem::multi(2947, ItemSlot::RangedOrRelic, 200), // Small Throwing Knife
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const NIGHT_ELF_PRIEST: &[StarterItem] = &[
    StarterItem::single(36, ItemSlot::MainHand), // Worn Mace
    StarterItem::single(6119, ItemSlot::Chest),  // Neophyte's Robe
    StarterItem::single(52, ItemSlot::Legs),     // Neophyte's Pants
    StarterItem::single(51, ItemSlot::Boots),    // Neophyte's Boots
    StarterItem::single(53, ItemSlot::Shirt),    // Neophyte's Shirt
    StarterItem::multi(2070, ItemSlot::Inventory0, 5), // Darnassian Bleu
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const NIGHT_ELF_DRUID: &[StarterItem] = &[
    StarterItem::single(3661, ItemSlot::MainHand), // Handcrafted Staff
    StarterItem::single(6123, ItemSlot::Chest),    // Novice's Robe
    StarterItem::single(6124, ItemSlot::Legs),     // Novice's Pants
    StarterItem::multi(159, ItemSlot::Inventory0, 5), // Refreshing Spring Water
    StarterItem::multi(4536, ItemSlot::Inventory1, 5), // Shiny Red Apple
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const UNDEAD_WARRIOR: &[StarterItem] = &[
    StarterItem::single(6125, ItemSlot::Shirt), // Brawler's Harness
    StarterItem::single(139, ItemSlot::Legs),   // Brawler's Pants
    StarterItem::single(140, ItemSlot::Boots),  // Brawler's Boots
    StarterItem::single(25, ItemSlot::MainHand), // Worn Shortsword
    StarterItem::single(2362, ItemSlot::OffHand), // Worn Wooden Shield
    StarterItem::multi(4604, ItemSlot::Inventory0, 5), // Forest Mushroom Cap
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const UNDEAD_ROGUE: &[StarterItem] = &[
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::single(2105, ItemSlot::Shirt),    // Thug Shirt
    StarterItem::single(120, ItemSlot::Legs),      // Thug Pants
    StarterItem::single(121, ItemSlot::Boots),     // Thug Boots
    StarterItem::multi(4604, ItemSlot::Inventory0, 5), // Forest Mushroom Cap
    StarterItem::multi(2947, ItemSlot::RangedOrRelic, 200), // Small Throwing Knife
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const UNDEAD_PRIEST: &[StarterItem] = &[
    StarterItem::single(6144, ItemSlot::Chest), // Neophyte's Robe
    StarterItem::single(53, ItemSlot::Shirt),   // Neophyte's Shirt
    StarterItem::single(52, ItemSlot::Legs),    // Neophyte's Pants
    StarterItem::single(36, ItemSlot::MainHand), // Worn Mace
    StarterItem::multi(4604, ItemSlot::Inventory0, 5), // Forest Mushroom Cap
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(51, ItemSlot::Boots),   // Neophyte's Boots
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const UNDEAD_MAGE: &[StarterItem] = &[
    StarterItem::single(6140, ItemSlot::Chest), // Apprentice's Robe
    StarterItem::single(1395, ItemSlot::Legs),  // Apprentice's Pants
    StarterItem::single(6096, ItemSlot::Shirt), // Apprentice's Shirt
    StarterItem::single(35, ItemSlot::MainHand), // Bent Staff
    StarterItem::single(55, ItemSlot::Boots),   // Apprentice's Boots
    StarterItem::multi(4604, ItemSlot::Inventory0, 5), // Forest Mushroom Cap
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const UNDEAD_WARLOCK: &[StarterItem] = &[
    StarterItem::single(6129, ItemSlot::Chest), // Acolyte's Robe
    StarterItem::single(1396, ItemSlot::Legs),  // Acolyte's Pants
    StarterItem::single(59, ItemSlot::Boots),   // Acolyte's Shoes
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::multi(4604, ItemSlot::Inventory0, 5), // Forest Mushroom Cap
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const TAUREN_WARRIOR: &[StarterItem] = &[
    StarterItem::single(6125, ItemSlot::Shirt), // Brawler's Harness
    StarterItem::single(139, ItemSlot::Legs),   // Brawler's Pants
    StarterItem::single(2361, ItemSlot::MainHand), // Battleworn Hammer
    StarterItem::single(6948, ItemSlot::Inventory0), // Hearthstone
    StarterItem::multi(4540, ItemSlot::Inventory1, 5), // Tough Hunk of Bread
];
const TAUREN_HUNTER: &[StarterItem] = &[
    StarterItem::single(127, ItemSlot::Shirt), // Trapper's Shirt
    StarterItem::single(6126, ItemSlot::Legs), // Trapper's Pants
    StarterItem::single(6948, ItemSlot::Inventory0), // Hearthstone
    StarterItem::single(37, ItemSlot::MainHand), // Worn Axe
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(2508, ItemSlot::RangedOrRelic), // Old Blunderbuss
    StarterItem::single(2102, ItemSlot::Bag1), // Small Ammo Pouch
    StarterItem::multi(2516, ItemSlot::Inventory3, 200), // Light Shot
    StarterItem::multi(117, ItemSlot::Inventory2, 5), // Tough Jerky
];
const TAUREN_SHAMAN: &[StarterItem] = &[
    StarterItem::single(154, ItemSlot::Shirt), // Primitive Mantle
    StarterItem::single(153, ItemSlot::Legs),  // Primitive Kilt
    StarterItem::single(6948, ItemSlot::Inventory0), // Hearthstone
    StarterItem::single(36, ItemSlot::MainHand), // Worn Mace
    StarterItem::multi(4604, ItemSlot::Inventory1, 5), // Forest Mushroom Cap
    StarterItem::multi(159, ItemSlot::Inventory2, 5), // Refreshing Spring Water
];
const TAUREN_DRUID: &[StarterItem] = &[
    StarterItem::single(35, ItemSlot::MainHand), // Bent Staff
    StarterItem::single(6139, ItemSlot::Chest),  // Novice's Robe
    StarterItem::single(6124, ItemSlot::Legs),   // Novice's Pants
    StarterItem::multi(159, ItemSlot::Inventory0, 5), // Refreshing Spring Water
    StarterItem::multi(4536, ItemSlot::Inventory1, 5), // Shiny Red Apple
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const GNOME_WARRIOR: &[StarterItem] = &[
    StarterItem::single(38, ItemSlot::Shirt), // Recruit's Shirt
    StarterItem::single(39, ItemSlot::Legs),  // Recruit's Pants
    StarterItem::single(40, ItemSlot::Boots), // Recruit's Boots
    StarterItem::single(25, ItemSlot::MainHand), // Worn Shortsword
    StarterItem::single(2362, ItemSlot::OffHand), // Worn Wooden Shield
    StarterItem::multi(117, ItemSlot::Inventory0, 5), // Tough Jerky
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const GNOME_ROGUE: &[StarterItem] = &[
    StarterItem::single(49, ItemSlot::Shirt), // Footpad's Shirt
    StarterItem::single(48, ItemSlot::Legs),  // Footpad's Pants
    StarterItem::single(47, ItemSlot::Boots), // Footpad's Shoes
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::multi(2947, ItemSlot::RangedOrRelic, 200), // Small Throwing Knife
    StarterItem::multi(117, ItemSlot::Inventory0, 5), // Tough Jerky
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const GNOME_MAGE: &[StarterItem] = &[
    StarterItem::single(56, ItemSlot::Chest), // Apprentice's Robe
    StarterItem::single(1395, ItemSlot::Legs), // Apprentice's Pants
    StarterItem::single(6096, ItemSlot::Shirt), // Apprentice's Shirt
    StarterItem::single(55, ItemSlot::Boots), // Apprentice's Boots
    StarterItem::single(35, ItemSlot::MainHand), // Bent Staff
    StarterItem::multi(4536, ItemSlot::Inventory0, 5), // Shiny Red Apple
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const GNOME_WARLOCK: &[StarterItem] = &[
    StarterItem::single(57, ItemSlot::Chest),   // Acolyte's Robe
    StarterItem::single(6097, ItemSlot::Shirt), // Acolyte's Shirt
    StarterItem::single(1396, ItemSlot::Legs),  // Acolyte's Pants
    StarterItem::single(59, ItemSlot::Boots),   // Acolyte's Shoes
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::multi(159, ItemSlot::Inventory0, 5), // Refreshing Spring Water
    StarterItem::multi(4604, ItemSlot::Inventory1, 5), // Forest Mushroom Cap
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const TROLL_WARRIOR: &[StarterItem] = &[
    StarterItem::single(37, ItemSlot::MainHand), // Worn Axe
    StarterItem::multi(117, ItemSlot::Inventory0, 5), // Tough Jerky
    StarterItem::single(6125, ItemSlot::Shirt),  // Brawler's Harness
    StarterItem::single(139, ItemSlot::Legs),    // Brawler's Pants
    StarterItem::single(2362, ItemSlot::OffHand), // Worn Wooden Shield
    StarterItem::multi(3111, ItemSlot::RangedOrRelic, 200), // Crude Throwing Axe
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const TROLL_HUNTER: &[StarterItem] = &[
    StarterItem::single(37, ItemSlot::MainHand), // Worn Axe
    StarterItem::multi(4604, ItemSlot::Inventory0, 5), // Forest Mushroom Cap
    StarterItem::single(2101, ItemSlot::Bag1),   // Light Quiver
    StarterItem::multi(2512, ItemSlot::Inventory3, 200), // Rough Arrow
    StarterItem::single(2504, ItemSlot::RangedOrRelic), // Worn Shortbow
    StarterItem::single(6126, ItemSlot::Legs),   // Trapper's Pants
    StarterItem::single(127, ItemSlot::Shirt),   // Trapper's Shirt
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const TROLL_ROGUE: &[StarterItem] = &[
    StarterItem::single(2092, ItemSlot::MainHand), // Worn Dagger
    StarterItem::multi(117, ItemSlot::Inventory0, 5), // Tough Jerky
    StarterItem::single(6136, ItemSlot::Shirt),    // Thug Shirt
    StarterItem::single(6137, ItemSlot::Legs),     // Thug Pants
    StarterItem::single(6138, ItemSlot::Boots),    // Thug Boots
    StarterItem::multi(3111, ItemSlot::RangedOrRelic, 200), // Crude Throwing Axe
    StarterItem::single(6948, ItemSlot::Inventory1), // Hearthstone
];
const TROLL_PRIEST: &[StarterItem] = &[
    StarterItem::single(36, ItemSlot::MainHand), // Worn Mace
    StarterItem::multi(4540, ItemSlot::Inventory0, 5), // Tough Hunk of Bread
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6144, ItemSlot::Chest),  // Neophyte's Robe
    StarterItem::single(53, ItemSlot::Shirt),    // Neophyte's Shirt
    StarterItem::single(52, ItemSlot::Legs),     // Neophyte's Pants
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const TROLL_SHAMAN: &[StarterItem] = &[
    StarterItem::single(36, ItemSlot::MainHand), // Worn Mace
    StarterItem::multi(117, ItemSlot::Inventory0, 5), // Tough Jerky
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6134, ItemSlot::Shirt),  // Primitive Mantle
    StarterItem::single(6135, ItemSlot::Legs),   // Primitive Kilt
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
const TROLL_MAGE: &[StarterItem] = &[
    StarterItem::single(6140, ItemSlot::Chest), // Apprentice's Robe
    StarterItem::single(1395, ItemSlot::Legs),  // Apprentice's Pants
    StarterItem::single(6096, ItemSlot::Shirt), // Apprentice's Shirt
    StarterItem::single(55, ItemSlot::Boots),   // Apprentice's Boots
    StarterItem::single(35, ItemSlot::MainHand), // Bent Staff
    StarterItem::multi(117, ItemSlot::Inventory0, 5), // Tough Jerky
    StarterItem::multi(159, ItemSlot::Inventory1, 5), // Refreshing Spring Water
    StarterItem::single(6948, ItemSlot::Inventory2), // Hearthstone
];
