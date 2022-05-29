# SMSG_PET_DISMISS_SOUND

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PET_DISMISS_SOUND = 0x0325 {
    u32 sound_id;
    f32 position_x;
    f32 position_y;
    f32 position_z;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | sound_id |  |  |
| 0x08 | 4 / Little | f32 | position_x |  |  |
| 0x0C | 4 / Little | f32 | position_y |  |  |
| 0x10 | 4 / Little | f32 | position_z |  |  |

