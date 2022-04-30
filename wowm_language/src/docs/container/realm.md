## Protocol Version 2, Protocol Version 3

### Wowm Representation
```rust,ignore
struct Realm {
    RealmType realm_type;    
    RealmFlag flag;    
    CString name;    
    CString address;    
    Population population;    
    u8 number_of_characters_on_realm;    
    RealmCategory category;    
    u8 realm_id;    
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | ? / - | RealmType | realm_type |  |
| - | ? / - | RealmFlag | flag |  |
| - | - / - | CString | name |  |
| - | - / - | CString | address |  |
| - | ? / - | Population | population |  |
| - | 1 / - | u8 | number_of_characters_on_realm |  |
| - | ? / - | RealmCategory | category |  |
| - | 1 / - | u8 | realm_id |  |
## Protocol Version 8

### Wowm Representation
```rust,ignore
struct Realm {
    u8 realm_type;    
    u8 locked;    
    RealmFlag flag;    
    CString name;    
    CString address;    
    Population population;    
    u8 number_of_characters_on_realm;    
    RealmCategory category;    
    u8 realm_id;    
    if (flag & SPECIFY_BUILD) {        
        Version version;        
    }    
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 1 / - | u8 | realm_type |  |
| 0x01 | 1 / - | u8 | locked |  |
| 0x02 | ? / - | RealmFlag | flag |  |
| - | - / - | CString | name |  |
| - | - / - | CString | address |  |
| - | ? / - | Population | population |  |
| - | 1 / - | u8 | number_of_characters_on_realm |  |
| - | ? / - | RealmCategory | category |  |
| - | 1 / - | u8 | realm_id |  |

If flag contains `SPECIFY_BUILD`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | Version | version |  |
