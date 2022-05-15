## Client Version 1.12

### Wowm Representation
```rust,ignore
struct GossipItem {
    u32 id;
    u8 item_icon;
    u8 coded;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | id |  | vmangos: sets to loop index |
| 0x04 | 1 / - | u8 | item_icon |  |  |
| 0x05 | 1 / - | u8 | coded |  | vmangos: makes pop up box password |
