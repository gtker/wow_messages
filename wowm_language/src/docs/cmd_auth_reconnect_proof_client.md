## Protocol Version 2, Protocol Version 8

### Wowm Representation
```rust,ignore
clogin CMD_AUTH_RECONNECT_PROOF_Client = 0x03 {
    u8[16] proof_data;
    u8[20] client_proof;
    u8[20] client_checksum;
    u8 key_count = 0;
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
| 0x01 | ? / - | u8[16] | proof_data |  |  |
| - | ? / - | u8[20] | client_proof |  |  |
| - | ? / - | u8[20] | client_checksum |  |  |
| - | 1 / - | u8 | key_count |  |  |
### Examples
```c
3, // opcode (3)
234, 250, 185, 198, 24, 21, 11, 242, 249, 50, 206, 39, 98, 121, 150, 153, // proof_data: u8[16]
107, 109, 26, 13, 243, 165, 158, 106, 56, 2, 231, 11, 225, 47, 5, 113, 186, 71, 140, 
163, // client_proof: u8[20]
40, 167, 158, 154, 36, 40, 230, 130, 237, 236, 199, 201, 232, 110, 241, 59, 123, 
225, 224, 245, // client_checksum: u8[20]
0, // key_count: u8
```
