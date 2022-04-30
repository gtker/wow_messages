## Protocol Version 2

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_RECONNECT_PROOF_Server = 0x03 {
    LoginResult result;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [LoginResult](loginresult.md) | result |  |
### Examples
```c
3, // opcode (3)
0, // result: LoginResult SUCCESS (0x00)
```
```c
3, // opcode (3)
14, // result: LoginResult SUCCESS_SURVEY (0x0E)
```
```c
3, // opcode (3)
14, // result: LoginResult SUCCESS_SURVEY (0x0E)
```
## Protocol Version 8

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_RECONNECT_PROOF_Server = 0x03 {
    LoginResult result;
    u16 padding = 0;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [LoginResult](loginresult.md) | result |  |
| - | 2 / Little | u16 | padding |  |
### Examples
```c
3, // opcode (3)
0, // result: LoginResult SUCCESS (0x00)
0, 0, // padding: u16
```
```c
3, // opcode (3)
16, // result: LoginResult FAIL_LOCKED_ENFORCED (0x10)
0, 0, // padding: u16
```
