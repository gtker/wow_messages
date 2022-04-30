## Protocol Version *

### Wowm Representation
```rust,ignore
clogin CMD_AUTH_LOGON_CHALLENGE_Client = 0x00 {
    u8 protocol_version;
    u16 size = self.size;
    u32 game_name = "\0WoW";
    Version version;
    Platform platform;
    Os os;
    Locale locale;
    u32 utc_timezone_offset;
    u32_be client_ip_address;
    u8 account_name_length;
    String[account_name_length] account_name;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 1 / - | u8 | protocol_version |  |
| 0x01 | 2 / Little | u16 | size |  |
| 0x03 | 4 / Little | u32 | game_name |  |
| 0x07 | ? / - | [Version](version.md) | version |  |
| - | ? / - | [Platform](platform.md) | platform |  |
| - | ? / - | [Os](os.md) | os |  |
| - | ? / - | [Locale](locale.md) | locale |  |
| - | 4 / Little | u32 | utc_timezone_offset | Offset in minutes from UTC time. 180 would be UTC+3 |
| - | 4 / Big | u32_be | client_ip_address |  |
| - | 1 / - | u8 | account_name_length |  |
| - | account_name_length / - | String[account_name_length] | account_name |  |
### Examples
```c
0, // opcode (0)
3, // protocol_version: u8
31, 0, // size: u16
87, 111, 87, 0, // game_name: u32
1, // Version.major: u8
12, // Version.minor: u8
1, // Version.patch: u8
243, 22, // Version.build: u16
54, 56, 120, 0, // platform: Platform X86 ("\0x86")
110, 105, 87, 0, // os: Os WINDOWS ("\0Win")
66, 71, 110, 101, // locale: Locale EN_GB ("enGB")
60, 0, 0, 0, // utc_timezone_offset: u32
127, 0, 0, 1, // client_ip_address: u32_be
1, // account_name_length: u8
65, // account_name: String[account_name_length]
```
