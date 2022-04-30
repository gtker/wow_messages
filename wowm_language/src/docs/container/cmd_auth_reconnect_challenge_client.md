## Protocol Version *

### Wowm Representation
```rust,ignore
clogin CMD_AUTH_RECONNECT_CHALLENGE_Client = 0x02 {
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
| 0x07 | ? / - | Version | version |  |
| - | ? / - | Platform | platform |  |
| - | ? / - | Os | os |  |
| - | ? / - | Locale | locale |  |
| - | 4 / Little | u32 | utc_timezone_offset |  |
| - | 4 / Big | u32_be | client_ip_address |  |
| - | 1 / - | u8 | account_name_length |  |
| - | account_name_length / - | String[account_name_length] | account_name |  |
