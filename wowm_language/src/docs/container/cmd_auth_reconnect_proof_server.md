## Protocol Version 2

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_RECONNECT_PROOF_Server = 0x03 {
    LoginResult result;    
}

```
## Protocol Version 8

### Wowm Representation
```rust,ignore
slogin CMD_AUTH_RECONNECT_PROOF_Server = 0x03 {
    LoginResult result;    
    u16 padding = 0;    
}

```
