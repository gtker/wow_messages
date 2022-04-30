## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_MOVE_KNOCK_BACK = 0x00EF {
    PackedGuid guid;    
    u32 movement_counter;    
    f32 v_cos;    
    f32 v_sin;    
    f32 horizontal_speed;    
    f32 vertical_speed;    
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
| 0x04 | - / - | PackedGuid | guid |  |
| - | 4 / Little | u32 | movement_counter |  |
| - | 4 / Little | f32 | v_cos |  |
| - | 4 / Little | f32 | v_sin |  |
| - | 4 / Little | f32 | horizontal_speed |  |
| - | 4 / Little | f32 | vertical_speed |  |
