## Client Version 1.12

### Wowm Representation
```rust,ignore
struct StabledPet {
    u32 pet_number;    
    u32 entry;    
    u32 level;    
    CString name;    
    u32 loyalty;    
    u8 slot;    
}

```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 4 / Little | u32 | pet_number |  |
| 0x04 | 4 / Little | u32 | entry |  |
| 0x08 | 4 / Little | u32 | level |  |
| 0x0C | - / - | CString | name |  |
| - | 4 / Little | u32 | loyalty |  |
| - | 1 / - | u8 | slot |  |
