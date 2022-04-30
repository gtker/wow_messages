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
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [LoginResult](loginresult.md) | login_result |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[20] | server_proof |  |
| - | 4 / Little | u32 | hardware_survey_id |  |
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
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | [LoginResult](loginresult.md) | login_result |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[20] | server_proof |  |
| - | ? / - | [AccountFlag](accountflag.md) | account_flag |  |
| - | 4 / Little | u32 | hardware_survey_id |  |
| - | 2 / Little | u16 | unknown_flags |  |

Else: 
| - | 2 / Little | u16 | padding |  |
