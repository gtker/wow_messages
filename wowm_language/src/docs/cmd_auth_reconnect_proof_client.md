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
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | u8[16] | proof_data |  |
| - | ? / - | u8[20] | client_proof |  |
| - | ? / - | u8[20] | client_checksum |  |
| - | 1 / - | u8 | key_count |  |
