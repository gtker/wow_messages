## Client Version 1.12

### Wowm Representation
```rust,ignore
struct GroupListMember {
    CString name;
    Guid guid;
    u8 is_online;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | - / - | CString | name |  |  |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| - | 1 / - | u8 | is_online |  |  |

