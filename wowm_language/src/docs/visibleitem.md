# VisibleItem

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L14).
```rust,ignore
struct VisibleItem {
    Guid creator;
    Item item;
    u32[2] enchants;
    u32 padding5 = 0;
    u32 padding6 = 0;
    u32 padding7 = 0;
    u32 padding8 = 0;
    u32 padding9 = 0;
    u32 random_property_id;
    u32 item_suffix_factor;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 8 / Little | [Guid](../types/packed-guid.md) | creator |  |
| 0x08 | 4 / Little | Item | item |  |
| 0x0C | 8 / - | u32[2] | enchants |  |
| 0x14 | 4 / Little | u32 | padding5 |  |
| 0x18 | 4 / Little | u32 | padding6 |  |
| 0x1C | 4 / Little | u32 | padding7 |  |
| 0x20 | 4 / Little | u32 | padding8 |  |
| 0x24 | 4 / Little | u32 | padding9 |  |
| 0x28 | 4 / Little | u32 | random_property_id |  |
| 0x2C | 4 / Little | u32 | item_suffix_factor |  |


Used in:

## Client Version 2.4.3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L29).
```rust,ignore
struct VisibleItem {
    Guid creator;
    Item item;
    u32[6] enchants;
    u32 random_property_id;
    u32 item_suffix_factor;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 8 / Little | [Guid](../types/packed-guid.md) | creator |  |
| 0x08 | 4 / Little | Item | item |  |
| 0x0C | 24 / - | u32[6] | enchants |  |
| 0x24 | 4 / Little | u32 | random_property_id |  |
| 0x28 | 4 / Little | u32 | item_suffix_factor |  |


Used in:

## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/update_mask/skill_info.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/update_mask/skill_info.wowm#L39).
```rust,ignore
struct VisibleItem {
    Item item;
    u16[2] enchants;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x00 | 4 / Little | Item | item |  |
| 0x04 | 4 / - | u16[2] | enchants |  |


Used in:

