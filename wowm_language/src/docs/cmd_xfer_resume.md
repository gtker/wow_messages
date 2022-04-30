## Protocol Version 3

### Wowm Representation
```rust,ignore
clogin CMD_XFER_RESUME = 0x33 {
    u64 offset;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 8 / Little | u64 | offset |  |
### Examples
```c
51, // opcode (51)
173, 222, 0, 0, 0, 0, 0, 0, // offset: u64
```
