## Protocol Version 2, Protocol Version 3

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
## Protocol Version 8

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
