## Protocol Version 3

### Wowm Representation
```rust,ignore
clogin CMD_XFER_RESUME = 0x33 {
    u64 offset;
}
```
### Header

Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x01 | 8 / Little | u64 | offset |  |  |

### Examples
```c
51, // opcode (51)
173, 222, 0, 0, 0, 0, 0, 0, // offset: u64
```
