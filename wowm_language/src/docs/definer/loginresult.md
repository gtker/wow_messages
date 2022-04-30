## Protocol Version 2, Protocol Version 3

## Wowm Representation
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
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| SUCCESS | 0x00 | 0 | 0x0 |  |  |
| FAIL_UNKNOWN0 | 0x01 | 1 | 0x1 |  |  |
| FAIL_UNKNOWN1 | 0x02 | 2 | 0x2 |  |  |
| FAIL_BANNED | 0x03 | 3 | 0x3 |  |  |
| FAIL_UNKNOWN_ACCOUNT | 0x04 | 4 | 0x4 |  |  |
| FAIL_INCORRECT_PASSWORD | 0x05 | 5 | 0x5 |  |  |
| FAIL_ALREADY_ONLINE | 0x06 | 6 | 0x6 |  |  |
| FAIL_NO_TIME | 0x07 | 7 | 0x7 |  |  |
| FAIL_DB_BUSY | 0x08 | 8 | 0x8 |  |  |
| FAIL_VERSION_INVALID | 0x09 | 9 | 0x9 |  |  |
| LOGIN_DOWNLOAD_FILE | 0x0A | 10 | 0xA |  |  |
| FAIL_INVALID_SERVER | 0x0B | 11 | 0xB |  |  |
| FAIL_SUSPENDED | 0x0C | 12 | 0xC |  |  |
| FAIL_NO_ACCESS | 0x0D | 13 | 0xD |  |  |
| SUCCESS_SURVEY | 0x0E | 14 | 0xE |  |  |
| FAIL_PARENTALCONTROL | 0x0F | 15 | 0xF |  |  |
## Protocol Version 8

## Wowm Representation
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
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| SUCCESS | 0x00 | 0 | 0x0 |  |  |
| FAIL_UNKNOWN0 | 0x01 | 1 | 0x1 |  |  |
| FAIL_UNKNOWN1 | 0x02 | 2 | 0x2 |  |  |
| FAIL_BANNED | 0x03 | 3 | 0x3 |  |  |
| FAIL_UNKNOWN_ACCOUNT | 0x04 | 4 | 0x4 |  |  |
| FAIL_INCORRECT_PASSWORD | 0x05 | 5 | 0x5 |  |  |
| FAIL_ALREADY_ONLINE | 0x06 | 6 | 0x6 |  |  |
| FAIL_NO_TIME | 0x07 | 7 | 0x7 |  |  |
| FAIL_DB_BUSY | 0x08 | 8 | 0x8 |  |  |
| FAIL_VERSION_INVALID | 0x09 | 9 | 0x9 |  |  |
| LOGIN_DOWNLOAD_FILE | 0x0A | 10 | 0xA |  |  |
| FAIL_INVALID_SERVER | 0x0B | 11 | 0xB |  |  |
| FAIL_SUSPENDED | 0x0C | 12 | 0xC |  |  |
| FAIL_NO_ACCESS | 0x0D | 13 | 0xD |  |  |
| SUCCESS_SURVEY | 0x0E | 14 | 0xE |  |  |
| FAIL_PARENTALCONTROL | 0x0F | 15 | 0xF |  |  |
| FAIL_LOCKED_ENFORCED | 0x10 | 16 | 0x10 |  |  |
