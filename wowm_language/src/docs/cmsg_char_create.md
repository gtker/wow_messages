## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_CHAR_CREATE = 0x0036 {
    CString name;
    Race race;
    Class class;
    Gender gender;
    u8 skin;
    u8 face;
    u8 hairstyle;
    u8 haircolor;
    u8 facialhair;
    u8 outfit_id;
}
```
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x06 | - / - | CString | name |  |
| - | ? / - | Race | race |  |
| - | ? / - | Class | class |  |
| - | ? / - | Gender | gender |  |
| - | 1 / - | u8 | skin |  |
| - | 1 / - | u8 | face |  |
| - | 1 / - | u8 | hairstyle |  |
| - | 1 / - | u8 | haircolor |  |
| - | 1 / - | u8 | facialhair |  |
| - | 1 / - | u8 | outfit_id |  |
