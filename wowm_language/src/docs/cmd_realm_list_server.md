## Protocol Version 2, Protocol Version 3

### Wowm Representation
```rust,ignore
slogin CMD_REALM_LIST_Server = 0x10 {
    u16 size = self.size;
    u32 header_padding = 0;
    u8 number_of_realms;
    Realm[number_of_realms] realms;
    u16 footer_padding = 0;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 2 / Little | u16 | size |  |
| 0x02 | 4 / Little | u32 | header_padding |  |
| 0x06 | 1 / - | u8 | number_of_realms |  |
| 0x07 | ? / - | Realm[number_of_realms] | realms |  |
| - | 2 / Little | u16 | footer_padding |  |
## Protocol Version 8

### Wowm Representation
```rust,ignore
slogin CMD_REALM_LIST_Server = 0x10 {
    u16 size = self.size;
    u32 header_padding = 0;
    u16 number_of_realms;
    Realm[number_of_realms] realms;
    u16 footer_padding = 0;
}
```
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 2 / Little | u16 | size |  |
| 0x02 | 4 / Little | u32 | header_padding |  |
| 0x06 | 2 / Little | u16 | number_of_realms |  |
| 0x08 | ? / - | Realm[number_of_realms] | realms |  |
| - | 2 / Little | u16 | footer_padding |  |
