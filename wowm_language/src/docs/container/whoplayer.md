## Client Version 1.12

### Wowm Representation
```rust,ignore
struct WhoPlayer {
    CString name;    
    CString guild;    
    u32 level;    
    Class class;    
    Race race;    
    u32 zone_id;    
    u32 party_status;    
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | - / - | CString | name |  |
| - | - / - | CString | guild |  |
| - | 4 / Little | u32 | level |  |
| - | ? / - | Class | class |  |
| - | ? / - | Race | race |  |
| - | 4 / Little | u32 | zone_id |  |
| - | 4 / Little | u32 | party_status |  |
