# SMSG_NAME_QUERY_RESPONSE
## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_NAME_QUERY_RESPONSE = 0x0051 {
    Guid guid;
    CString character_name;
    CString realm_name;
    Race race;
    Gender gender;
    Class class;
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |
| 0x0C | - / - | CString | character_name |  |
| - | - / - | CString | realm_name |  |
| - | ? / - | [Race](race.md) | race |  |
| - | ? / - | [Gender](gender.md) | gender |  |
| - | ? / - | [Class](class.md) | class |  |
### Examples
```c
0, 28, // size
81, 0, // opcode (81)
239, 190, 173, 222, 0, 0, 0, 0, // guid: Guid
65, 115, 100, 102, 0, // character_name: CString
0, // realm_name: CString
1, 0, 0, 0, // race: Race HUMAN (1)
0, 0, 0, 0, // gender: Gender MALE (0)
1, 0, 0, 0, // class: Class WARRIOR (1)
```
```c
0, 29, // size
81, 0, // opcode (81)
239, 190, 173, 222, 0, 0, 0, 0, // guid: Guid
65, 115, 100, 102, 0, // character_name: CString
65, 0, // realm_name: CString
1, 0, 0, 0, // race: Race HUMAN (1)
0, 0, 0, 0, // gender: Gender MALE (0)
1, 0, 0, 0, // class: Class WARRIOR (1)
```
