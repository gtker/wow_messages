#[cfg(feature = "tbc")]
pub mod tbc;
#[cfg(feature = "vanilla")]
pub mod vanilla;
#[cfg(feature = "wrath")]
pub mod wrath;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
macro_rules! find_and_data {
    ($data:expr) => {
        pub fn lookup_spell(id: u32) -> Option<&'static Spell> {
            all_spells().iter().find(|a| a.entry == id)
        }

        pub const fn all_spells() -> &'static [Spell] {
            $data
        }
    };
}
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) use find_and_data;
