## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_AUTH_SESSION = 0x01ED {
    u32 build;
    u32 server_id;
    CString username;
    u32 client_seed;
    u8[20] client_proof;
    u32 decompressed_addon_info_size;
    u8[-] compressed_addon_info;
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
| 0x06 | 4 / Little | u32 | build |  |  |
| 0x0A | 4 / Little | u32 | server_id |  |  |
| 0x0E | - / - | CString | username |  |  |
| - | 4 / Little | u32 | client_seed |  |  |
| - | ? / - | u8[20] | client_proof |  |  |
| - | 4 / Little | u32 | decompressed_addon_info_size |  |  |
| - | ? / - | u8[-] | compressed_addon_info |  |  |

