## Protocol Version 2, Protocol Version 3

```rust,ignore
flag RealmFlag : u8 {
    NONE = 0x00;    
    INVALID = 0x01;    
    OFFLINE = 0x02;    
    FORCE_BLUE_RECOMMENDED = 0x20;    
    FORCE_GREEN_RECOMMENDED = 0x40;    
    FORCE_RED_FULL = 0x80;    
}

```
## Protocol Version 8

```rust,ignore
flag RealmFlag : u8 {
    NONE = 0x00;    
    INVALID = 0x01;    
    OFFLINE = 0x02;    
    SPECIFY_BUILD = 0x04;    
    FORCE_BLUE_RECOMMENDED = 0x20;    
    FORCE_GREEN_RECOMMENDED = 0x40;    
    FORCE_RED_FULL = 0x80;    
}

```
