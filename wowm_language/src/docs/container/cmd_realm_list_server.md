## Protocol Version 2, Protocol Version 3

### Wowm Representation
```rust,ignore
slogin CMD_REALM_LIST_Server = 0x10 {
    u16 size = self.size;    
    u32 header_padding = 0;    
    u8 number_of_realms;    
    Realm[number_of_realms] realms;    
    u16 footer_padding = 0;    
}

```
## Protocol Version 8

### Wowm Representation
```rust,ignore
slogin CMD_REALM_LIST_Server = 0x10 {
    u16 size = self.size;    
    u32 header_padding = 0;    
    u16 number_of_realms;    
    Realm[number_of_realms] realms;    
    u16 footer_padding = 0;    
}

```
