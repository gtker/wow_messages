## Protocol Version 3

```rust,ignore
slogin CMD_XFER_DATA = 0x31 {
    u16 size;    
    u8[size] data;    
}

```
