## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LOOT_START_ROLL = 0x02A1 {
    Guid creature_guid;
    u32 loot_slot;
    u32 item_id;
    u32 item_random_suffix;
    u32 item_random_property_id;
    u32 countdown_time;
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | 8 / Little | Guid | creature_guid |  |
| 0x0C | 4 / Little | u32 | loot_slot |  |
| 0x10 | 4 / Little | u32 | item_id |  |
| 0x14 | 4 / Little | u32 | item_random_suffix |  |
| 0x18 | 4 / Little | u32 | item_random_property_id |  |
| 0x1C | 4 / Little | u32 | countdown_time |  |
