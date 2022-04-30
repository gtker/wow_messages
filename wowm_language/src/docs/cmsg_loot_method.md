## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_LOOT_METHOD = 0x007A {
    GroupLootSetting loot_setting;
    Guid loot_master;
    ItemQuality loot_threshold;
}
```
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x06 | ? / - | [GroupLootSetting](grouplootsetting.md) | loot_setting |  |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | loot_master |  |
| - | ? / - | [ItemQuality](itemquality.md) | loot_threshold |  |
