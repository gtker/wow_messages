use crate::extended::shared::{BLUE, GREEN, GREY, LIGHT_YELLOW, ORANGE, PURPLE, WHITE};
use crate::shared::item_quality_vanilla_tbc::ItemQuality;

impl ItemQuality {
    /// Returns the item name color for the quality.
    ///
    /// Format is Alpha (index 0), Red (index 1), Green (index 2), Blue (index 3) (ARGB).
    /// The alpha is always `0xFF` (255).
    pub const fn text_color(&self) -> [u8; 4] {
        match self {
            ItemQuality::Poor => GREY,
            ItemQuality::Normal => WHITE,
            ItemQuality::Uncommon => GREEN,
            ItemQuality::Rare => BLUE,
            ItemQuality::Epic => PURPLE,
            ItemQuality::Legendary => ORANGE,
            ItemQuality::Artifact => LIGHT_YELLOW,
        }
    }
}
