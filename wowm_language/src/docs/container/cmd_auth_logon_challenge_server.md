## Protocol Version 2

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_LOGON_CHALLENGE_Server = 0x00 {
    u8 protocol_version = 0;    
    LoginResult login_result;    
    if (login_result == SUCCESS) {        
        u8[32] server_public_key;        
        u8 generator_length;        
        u8[generator_length] generator;        
        u8 large_safe_prime_length;        
        u8[large_safe_prime_length] large_safe_prime;        
        u8[32] salt;        
        u8[16] crc_salt;        
    }    
}

```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 1 / - | u8 | protocol_version |  |
| 0x01 | ? / - | LoginResult | login_result |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[32] | server_public_key |  |
| - | 1 / - | u8 | generator_length |  |
| - | ? / - | u8[generator_length] | generator |  |
| - | 1 / - | u8 | large_safe_prime_length |  |
| - | ? / - | u8[large_safe_prime_length] | large_safe_prime |  |
| - | ? / - | u8[32] | salt |  |
| - | ? / - | u8[16] | crc_salt |  |
## Protocol Version 3

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_LOGON_CHALLENGE_Server = 0x00 {
    u8 protocol_version = 0;    
    LoginResult login_result;    
    if (login_result == SUCCESS) {        
        u8[32] server_public_key;        
        u8 generator_length;        
        u8[generator_length] generator;        
        u8 large_safe_prime_length;        
        u8[large_safe_prime_length] large_safe_prime;        
        u8[32] salt;        
        u8[16] crc_salt;        
        SecurityFlag security_flag;        
        if (security_flag == PIN) {            
            u32 pin_grid_seed;            
            u8[16] pin_salt;            
        }        
    }    
}

```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 1 / - | u8 | protocol_version |  |
| 0x01 | ? / - | LoginResult | login_result |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[32] | server_public_key |  |
| - | 1 / - | u8 | generator_length |  |
| - | ? / - | u8[generator_length] | generator |  |
| - | 1 / - | u8 | large_safe_prime_length |  |
| - | ? / - | u8[large_safe_prime_length] | large_safe_prime |  |
| - | ? / - | u8[32] | salt |  |
| - | ? / - | u8[16] | crc_salt |  |
| - | ? / - | SecurityFlag | security_flag |  |

If security_flag is equal to `PIN`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | pin_grid_seed |  |
| - | ? / - | u8[16] | pin_salt |  |
## Protocol Version 8

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_LOGON_CHALLENGE_Server = 0x00 {
    u8 protocol_version = 0;    
    LoginResult login_result;    
    if (login_result == SUCCESS) {        
        u8[32] server_public_key;        
        u8 generator_length;        
        u8[generator_length] generator;        
        u8 large_safe_prime_length;        
        u8[large_safe_prime_length] large_safe_prime;        
        u8[32] salt;        
        u8[16] crc_salt;        
        SecurityFlag security_flag;        
        if (security_flag & PIN) {            
            u32 pin_grid_seed;            
            u8[16] pin_salt;            
        }        
        if (security_flag & UNKNOWN0) {            
            u8 unknown0;            
            u8 unknown1;            
            u8 unknown2;            
            u8 unknown3;            
            u64 unknown4;            
        }        
        if (security_flag & AUTHENTICATOR) {            
            u8 unknown5;            
        }        
    }    
}

```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 1 / - | u8 | protocol_version |  |
| 0x01 | ? / - | LoginResult | login_result |  |

If login_result is equal to `SUCCESS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | u8[32] | server_public_key |  |
| - | 1 / - | u8 | generator_length |  |
| - | ? / - | u8[generator_length] | generator |  |
| - | 1 / - | u8 | large_safe_prime_length |  |
| - | ? / - | u8[large_safe_prime_length] | large_safe_prime |  |
| - | ? / - | u8[32] | salt |  |
| - | ? / - | u8[16] | crc_salt |  |
| - | ? / - | SecurityFlag | security_flag |  |

If security_flag contains `PIN`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | pin_grid_seed |  |
| - | ? / - | u8[16] | pin_salt |  |

If security_flag contains `UNKNOWN0`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | unknown0 |  |
| - | 1 / - | u8 | unknown1 |  |
| - | 1 / - | u8 | unknown2 |  |
| - | 1 / - | u8 | unknown3 |  |
| - | 8 / Little | u64 | unknown4 |  |

If security_flag contains `AUTHENTICATOR`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | unknown5 |  |
