## Protocol Version 3

### Wowm Representation
```rust,ignore
clogin CMD_SURVEY_RESULT = 0x04 {
    u32 survey_id;
    u8 error;
    u16 compressed_data_length;
    u8[compressed_data_length] data;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | survey_id |  |
| 0x04 | 1 / - | u8 | error |  |
| 0x05 | 2 / Little | u16 | compressed_data_length |  |
| 0x07 | ? / - | u8[compressed_data_length] | data |  |
### Examples
```c
4, // opcode (4)
222, 250, 0, 0, // survey_id: u32
0, // error: u8
1, 0, // compressed_data_length: u16
UNIMPLEMENTED_DOC_ARRAY
// data: u8[compressed_data_length]
```
