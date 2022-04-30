## Protocol Version 3

### Wowm Representation
```rust,ignore
slogin CMD_XFER_DATA = 0x31 {
    u16 size;
    u8[size] data;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 2 / Little | u16 | size |  |
| 0x02 | ? / - | u8[size] | data |  |
### Examples
```c
49, // opcode (49)
1, 0, // size: u16
255, // data: u8[size]
```
```c
49, // opcode (49)
0, 0, // size: u16
// data: u8[size]
```
