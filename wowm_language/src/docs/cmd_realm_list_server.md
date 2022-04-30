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
| 0x07 | ? / - | [Realm](realm.md)[number_of_realms] | realms |  |
| - | 2 / Little | u16 | footer_padding |  |
### Examples
```c
16, // opcode (16)
23, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, // number_of_realms: u8
0, 0, 0, 0, // [0].Realm.realm_type: RealmType PLAYER_VS_ENVIRONMENT (0)
0, // [1].Realm.flag: RealmFlag  NONE (0)
65, 0, // [2].Realm.name: CString
65, 0, // [3].Realm.address: CString
0, 0, 200, 67, // [4].Realm.population: Population RED_FULL (0x43c80000)
1, // [5].Realm.number_of_characters_on_realm: u8
0, // [6].Realm.category: RealmCategory DEFAULT (0x0)
2, // [7].Realm.realm_id: u8
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```
```c
16, // opcode (16)
23, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, // number_of_realms: u8
0, 0, 0, 0, // [0].Realm.realm_type: RealmType PLAYER_VS_ENVIRONMENT (0)
3, // [1].Realm.flag: RealmFlag  INVALID| OFFLINE (3)
65, 0, // [2].Realm.name: CString
65, 0, // [3].Realm.address: CString
0, 0, 200, 67, // [4].Realm.population: Population RED_FULL (0x43c80000)
1, // [5].Realm.number_of_characters_on_realm: u8
0, // [6].Realm.category: RealmCategory DEFAULT (0x0)
2, // [7].Realm.realm_id: u8
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```
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
| 0x08 | ? / - | [Realm](realm.md)[number_of_realms] | realms |  |
| - | 2 / Little | u16 | footer_padding |  |
### Examples
```c
16, // opcode (16)
22, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, 0, // number_of_realms: u16
0, // [0].Realm.realm_type: u8
0, // [1].Realm.locked: u8
3, // [2].Realm.flag: RealmFlag  INVALID| OFFLINE (3)
65, 0, // [3].Realm.name: CString
65, 0, // [4].Realm.address: CString
0, 0, 200, 67, // [5].Realm.population: Population RED_FULL (0x43c80000)
1, // [6].Realm.number_of_characters_on_realm: u8
0, // [7].Realm.category: RealmCategory DEFAULT (0x0)
2, // [8].Realm.realm_id: u8
UNIMPLEMENTED_DOC_IF
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```
```c
16, // opcode (16)
27, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, 0, // number_of_realms: u16
0, // [0].Realm.realm_type: u8
0, // [1].Realm.locked: u8
4, // [2].Realm.flag: RealmFlag  SPECIFY_BUILD (4)
65, 0, // [3].Realm.name: CString
65, 0, // [4].Realm.address: CString
0, 0, 200, 67, // [5].Realm.population: Population RED_FULL (0x43c80000)
1, // [6].Realm.number_of_characters_on_realm: u8
0, // [7].Realm.category: RealmCategory DEFAULT (0x0)
2, // [8].Realm.realm_id: u8
UNIMPLEMENTED_DOC_IF
// realms: Realm[number_of_realms]
1, 12, // footer_padding: u16
```
