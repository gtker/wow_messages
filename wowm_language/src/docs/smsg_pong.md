## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PONG = 0x01DD {
    u32 sequence_id;
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
| 0x04 | 4 / Little | u32 | sequence_id |  |  |

### Examples
```c
0, 6, // size
221, 1, // opcode (477)
239, 190, 173, 222, // sequence_id: u32
```
