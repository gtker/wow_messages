use super::print_specific_update_mask;
use super::wrath_fields;
use crate::parser::types::version::MajorWorldVersion;
use crate::parser::types::IntegerType;
use crate::rust_printer::UpdateMaskDataType;

#[test]
fn wrath_buyback_fields_are_indexed_u32_arrays() {
    for (name, offset) in [("BUYBACK_PRICE", 0x04B1), ("BUYBACK_TIMESTAMP", 0x04BD)] {
        let field = wrath_fields::FIELDS
            .iter()
            .find(|field| field.name() == name)
            .unwrap();

        assert_eq!(field.offset(), offset);
        assert_eq!(field.size(), 12);
        assert!(matches!(
            field.ty(),
            UpdateMaskDataType::ArrayOfInteger {
                integer_type: IntegerType::U32,
                name: "BuybackSlot",
                variable_name: "buyback_slot",
                import_location: "crate::wrath",
                index_origin: 74,
            }
        ));
    }
}

#[test]
fn wrath_buyback_accessors_use_typed_slots_and_slot_relative_offsets() {
    let output = print_specific_update_mask(wrath_fields::FIELDS, MajorWorldVersion::Wrath);
    let output = output.inner();

    for field in ["buyback_price", "buyback_timestamp"] {
        assert!(output.contains(&format!(
            "pub fn set_player_{field}(mut self, buyback_slot: crate::wrath::BuybackSlot, v: u32) -> Self"
        )));
        assert!(output.contains(&format!(
            "pub fn set_player_{field}(&mut self, buyback_slot: crate::wrath::BuybackSlot, v: u32)"
        )));
        assert!(output.contains(&format!(
            "pub fn player_{field}(&self, buyback_slot: crate::wrath::BuybackSlot) -> Option<u32>"
        )));
    }

    assert!(output.contains("let offset = 1201 + buyback_slot.as_int() as u16 - 74;"));
    assert!(output.contains("let offset = 1213 + buyback_slot.as_int() as u16 - 74;"));
}
