#[cfg(feature = "tbc")]
pub mod tbc;
#[cfg(feature = "vanilla")]
pub mod vanilla;
#[cfg(feature = "wrath")]
pub mod wrath;

macro_rules! find_and_data {
    ($data:expr) => {
        pub fn lookup_item(id: u32) -> Option<&'static Item> {
            all_items().iter().find(|a| a.entry == id)
        }

        pub const fn all_items() -> &'static [Item] {
            $data
        }
    };
}
pub(crate) use find_and_data;
