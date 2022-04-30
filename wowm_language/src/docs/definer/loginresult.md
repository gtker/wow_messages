## Protocol Version 2, Protocol Version 3

### Wowm Representation
```rust,ignore
enum LoginResult : u8 {
    SUCCESS = 0x00;
    FAIL_UNKNOWN0 = 0x01;
    FAIL_UNKNOWN1 = 0x02;
    FAIL_BANNED = 0x03;
    FAIL_UNKNOWN_ACCOUNT = 0x04;
    FAIL_INCORRECT_PASSWORD = 0x05;
    FAIL_ALREADY_ONLINE = 0x06;
    FAIL_NO_TIME = 0x07;
    FAIL_DB_BUSY = 0x08;
    FAIL_VERSION_INVALID = 0x09;
    LOGIN_DOWNLOAD_FILE = 0x0A;
    FAIL_INVALID_SERVER = 0x0B;
    FAIL_SUSPENDED = 0x0C;
    FAIL_NO_ACCESS = 0x0D;
    SUCCESS_SURVEY = 0x0E;
    FAIL_PARENTALCONTROL = 0x0F;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SUCCESS` | 0 (0x00) |  |  |
| `FAIL_UNKNOWN0` | 1 (0x01) |  |  |
| `FAIL_UNKNOWN1` | 2 (0x02) |  |  |
| `FAIL_BANNED` | 3 (0x03) |  |  |
| `FAIL_UNKNOWN_ACCOUNT` | 4 (0x04) |  |  |
| `FAIL_INCORRECT_PASSWORD` | 5 (0x05) |  |  |
| `FAIL_ALREADY_ONLINE` | 6 (0x06) |  |  |
| `FAIL_NO_TIME` | 7 (0x07) |  |  |
| `FAIL_DB_BUSY` | 8 (0x08) |  |  |
| `FAIL_VERSION_INVALID` | 9 (0x09) |  |  |
| `LOGIN_DOWNLOAD_FILE` | 10 (0x0A) |  |  |
| `FAIL_INVALID_SERVER` | 11 (0x0B) |  |  |
| `FAIL_SUSPENDED` | 12 (0x0C) |  |  |
| `FAIL_NO_ACCESS` | 13 (0x0D) |  |  |
| `SUCCESS_SURVEY` | 14 (0x0E) |  |  |
| `FAIL_PARENTALCONTROL` | 15 (0x0F) |  |  |
## Protocol Version 8

### Wowm Representation
```rust,ignore
enum LoginResult : u8 {
    SUCCESS = 0x00;
    FAIL_UNKNOWN0 = 0x01;
    FAIL_UNKNOWN1 = 0x02;
    FAIL_BANNED = 0x03;
    FAIL_UNKNOWN_ACCOUNT = 0x04;
    FAIL_INCORRECT_PASSWORD = 0x05;
    FAIL_ALREADY_ONLINE = 0x06;
    FAIL_NO_TIME = 0x07;
    FAIL_DB_BUSY = 0x08;
    FAIL_VERSION_INVALID = 0x09;
    LOGIN_DOWNLOAD_FILE = 0x0A;
    FAIL_INVALID_SERVER = 0x0B;
    FAIL_SUSPENDED = 0x0C;
    FAIL_NO_ACCESS = 0x0D;
    SUCCESS_SURVEY = 0x0E;
    FAIL_PARENTALCONTROL = 0x0F;
    FAIL_LOCKED_ENFORCED = 0x10;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SUCCESS` | 0 (0x00) |  |  |
| `FAIL_UNKNOWN0` | 1 (0x01) |  |  |
| `FAIL_UNKNOWN1` | 2 (0x02) |  |  |
| `FAIL_BANNED` | 3 (0x03) |  |  |
| `FAIL_UNKNOWN_ACCOUNT` | 4 (0x04) |  |  |
| `FAIL_INCORRECT_PASSWORD` | 5 (0x05) |  |  |
| `FAIL_ALREADY_ONLINE` | 6 (0x06) |  |  |
| `FAIL_NO_TIME` | 7 (0x07) |  |  |
| `FAIL_DB_BUSY` | 8 (0x08) |  |  |
| `FAIL_VERSION_INVALID` | 9 (0x09) |  |  |
| `LOGIN_DOWNLOAD_FILE` | 10 (0x0A) |  |  |
| `FAIL_INVALID_SERVER` | 11 (0x0B) |  |  |
| `FAIL_SUSPENDED` | 12 (0x0C) |  |  |
| `FAIL_NO_ACCESS` | 13 (0x0D) |  |  |
| `SUCCESS_SURVEY` | 14 (0x0E) |  |  |
| `FAIL_PARENTALCONTROL` | 15 (0x0F) |  |  |
| `FAIL_LOCKED_ENFORCED` | 16 (0x10) |  |  |
