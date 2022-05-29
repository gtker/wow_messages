## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_PET_SET_ACTION = 0x0174 {
    Guid guid;
    u32 position1;
    u32 data1;
    optional extra {
        u32 position2;
        u32 data2;
    }
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x0E | 4 / Little | u32 | position1 |  |  |
| 0x12 | 4 / Little | u32 | data1 |  |  |

Optionally the following fields can be present. This can only be detected by looking at the size of the message.

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x16 | 4 / Little | u32 | position2 |  |  |
| 0x1A | 4 / Little | u32 | data2 |  |  |

