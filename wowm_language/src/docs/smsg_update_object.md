## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_UPDATE_OBJECT = 0x00A9 {
    u32 amount_of_objects;
    u8 has_transport;
    Object[amount_of_objects] objects;
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
| 0x04 | 4 / Little | u32 | amount_of_objects |  |  |
| 0x08 | 1 / - | u8 | has_transport |  |  |
| 0x09 | ? / - | [Object](object.md)[amount_of_objects] | objects |  |  |
### Examples
```c
0, 97, // size
169, 0, // opcode (169)
1, 0, 0, 0, // amount_of_objects: u32
0, // has_transport: u8
3, // [0].Object.update_type: UpdateType CREATE_OBJECT2 (3)
4, // [1].Object.guid3: PackedGuid
4, // [1].Object.object_type: ObjectType PLAYER (4)
49, // MovementBlock.update_flag: UpdateFlag  SELF| ALL| LIVING (49)
0, 0, 0, 0, // MovementBlock.flags: MovementFlags  NONE (0)
0, 0, 0, 0, // MovementBlock.timestamp: u32
205, 215, 11, 198, // MovementBlock.living_position_x: f32
53, 126, 4, 195, // MovementBlock.living_position_y: f32
249, 15, 167, 66, // MovementBlock.living_position_z: f32
0, 0, 0, 0, // MovementBlock.living_orientation: f32
0, 0, 0, 0, // MovementBlock.fall_time: f32
0, 0, 128, 63, // MovementBlock.walking_speed: f32
0, 0, 224, 64, // MovementBlock.running_speed: f32
0, 0, 144, 64, // MovementBlock.backwards_running_speed: f32
0, 0, 0, 0, // MovementBlock.swimming_speed: f32
0, 0, 0, 0, // MovementBlock.backwards_swimming_speed: f32
219, 15, 73, 64, // MovementBlock.turn_rate: f32
1, 0, 0, 0, // MovementBlock.unknown1: u32
2, // amount_of_blocks
UPDATEMASK TODO
// [1].Object.mask2: UpdateMask
// objects: Object[amount_of_objects]
```
