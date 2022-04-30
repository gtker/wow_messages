## Protocol Version 2

## Wowm Representation
```rust,ignore
slogin CMD_AUTH_RECONNECT_CHALLENGE_Server = 0x02 {
    LoginResult result;    
    if (result == SUCCESS) {        
        u8[16] challenge_data;        
        u8[16] checksum_salt;        
    }    
}

```
## Protocol Version 8

## Wowm Representation
```rust,ignore
slogin CMD_AUTH_RECONNECT_CHALLENGE_Server = 0x02 {
    LoginResult result;    
    if (result == SUCCESS) {        
        u8[16] challenge_data;        
        u8[16] checksum_salt;        
    }    
}

```
