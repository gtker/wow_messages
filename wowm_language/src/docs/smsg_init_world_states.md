## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_INIT_WORLD_STATES = 0x02C2 {
    Map map;
    Area area;
    u16 amount_of_states;
    WorldState[amount_of_states] states;
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
| 0x04 | ? / - | [Map](map.md) | map |  |  |
| - | ? / - | [Area](area.md) | area |  |  |
| - | 2 / Little | u16 | amount_of_states |  |  |
| - | ? / - | [WorldState](worldstate.md)[amount_of_states] | states |  |  |

