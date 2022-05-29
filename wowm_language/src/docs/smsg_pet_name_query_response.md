# SMSG_PET_NAME_QUERY_RESPONSE

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PET_NAME_QUERY_RESPONSE = 0x0053 {
    u32 pet_number;
    CString name;
    u32 pet_name_timestamp;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | pet_number |  |  |
| 0x08 | - / - | CString | name |  |  |
| - | 4 / Little | u32 | pet_name_timestamp |  |  |

### Examples
```c
0, 17, // size
83, 0, // opcode (83)
239, 190, 173, 222, // pet_number: u32
65, 66, 67, 68, 69, 70, 0, // name: CString
222, 202, 250, 0, // pet_name_timestamp: u32
```
