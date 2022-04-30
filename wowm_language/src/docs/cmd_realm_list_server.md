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
UNIMPLEMENTED_DOC_ARRAY
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```
```c
16, // opcode (16)
23, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, // number_of_realms: u8
UNIMPLEMENTED_DOC_ARRAY
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
UNIMPLEMENTED_DOC_ARRAY
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```
```c
16, // opcode (16)
27, 0, // size: u16
0, 0, 0, 0, // header_padding: u32
1, 0, // number_of_realms: u16
UNIMPLEMENTED_DOC_ARRAY
// realms: Realm[number_of_realms]
0, 0, // footer_padding: u16
```
