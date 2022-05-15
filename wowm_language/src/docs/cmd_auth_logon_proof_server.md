## Protocol Version 2, Protocol Version 3

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_LOGON_PROOF_Server = 0x01 {
    LoginResult login_result;
    if (login_result == SUCCESS) {
        u8[20] server_proof;
        u32 hardware_survey_id;
    }
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
| 0x01 | ? / - | [LoginResult](loginresult.md) | login_result |  |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | u8[20] | server_proof |  |  |
| - | 4 / Little | u32 | hardware_survey_id |  |  |
### Examples
```c
1, // opcode (1)
0, // login_result: LoginResult SUCCESS (0x00)
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // server_proof: u8[20]
239, 190, 173, 222, // hardware_survey_id: u32
```
## Protocol Version 8

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_LOGON_PROOF_Server = 0x01 {
    LoginResult login_result;
    if (login_result == SUCCESS) {
        u8[20] server_proof;
        AccountFlag account_flag;
        u32 hardware_survey_id;
        u16 unknown_flags;
    }
    else {
        u16 padding = 0;
    }
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
| 0x01 | ? / - | [LoginResult](loginresult.md) | login_result |  |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | u8[20] | server_proof |  |  |
| - | ? / - | [AccountFlag](accountflag.md) | account_flag |  |  |
| - | 4 / Little | u32 | hardware_survey_id |  |  |
| - | 2 / Little | u16 | unknown_flags |  |  |

Else: 
| - | 2 / Little | u16 | padding |  |  |
### Examples
```c
1, // opcode (1)
7, // login_result: LoginResult FAIL_NO_TIME (0x07)
0, 0, // padding: u16
```
```c
1, // opcode (1)
8, // login_result: LoginResult FAIL_DB_BUSY (0x08)
0, 0, // padding: u16
```
