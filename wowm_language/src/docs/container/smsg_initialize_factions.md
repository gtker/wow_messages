## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_INITIALIZE_FACTIONS = 0x0122 {
    u32 amount_of_factions;    
    FactionInitializer[amount_of_factions] factions;    
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
| 0x04 | 4 / Little | u32 | amount_of_factions |  |
| 0x08 | ? / - | FactionInitializer[amount_of_factions] | factions |  |
