## Client Version 1.12

### Wowm Representation
```rust,ignore
struct ChannelMember {
    Guid guid;    
    u8 member_flags;    
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 8 / Little | Guid | guid |  |
| 0x08 | 1 / - | u8 | member_flags |  |
