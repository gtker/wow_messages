## Client Version 1.12

### Wowm Representation
```rust,ignore
struct Friend {
    Guid guid;
    FriendStatus status;
    if (status != OFFLINE) {
        Area area;
        u32 level;
        Class class;
    }
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x08 | ? / - | [FriendStatus](friendstatus.md) | status |  |  |

If status is not equal to `OFFLINE`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | [Area](area.md) | area |  |  |
| - | 4 / Little | u32 | level |  |  |
| - | ? / - | [Class](class.md) | class |  |  |

