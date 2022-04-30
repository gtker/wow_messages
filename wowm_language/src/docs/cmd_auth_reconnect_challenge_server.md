## Protocol Version 2

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_RECONNECT_CHALLENGE_Server = 0x02 {
    LoginResult result;
    if (result == SUCCESS) {
        u8[16] challenge_data;
        u8[16] checksum_salt;
    }
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [LoginResult](loginresult.md) | result |  |

If result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[16] | challenge_data |  |
| - | ? / - | u8[16] | checksum_salt |  |
### Examples
```c
2, // opcode (2)
0, // result: LoginResult SUCCESS (0x00)
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // challenge_data: u8[16]
255, 254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240, // checksum_salt: u8[16]
```
```c
2, // opcode (2)
3, // result: LoginResult FAIL_BANNED (0x03)
```
## Protocol Version 8

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_RECONNECT_CHALLENGE_Server = 0x02 {
    LoginResult result;
    if (result == SUCCESS) {
        u8[16] challenge_data;
        u8[16] checksum_salt;
    }
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [LoginResult](loginresult.md) | result |  |

If result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[16] | challenge_data |  |
| - | ? / - | u8[16] | checksum_salt |  |
### Examples
```c
2, // opcode (2)
0, // result: LoginResult SUCCESS (0x00)
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // challenge_data: u8[16]
255, 254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240, // checksum_salt: u8[16]
```
```c
2, // opcode (2)
3, // result: LoginResult FAIL_BANNED (0x03)
```
