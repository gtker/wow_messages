## Protocol Version 2, Protocol Version 3, Protocol Version 8

### Wowm Representation
```rust,ignore
clogin CMD_REALM_LIST_Client = 0x10 {
    u32 padding = 0;    
}

```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | padding |  |
