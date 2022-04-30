## Protocol Version *

```rust,ignore
clogin CMD_AUTH_LOGON_CHALLENGE_Client = 0x0 {
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
