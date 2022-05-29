# SMSG_PETITION_QUERY_RESPONSE

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PETITION_QUERY_RESPONSE = 0x01C7 {
    Guid petition_guid;
    Guid charter_owner;
    CString guild_name;
    CString body_text;
    u32 unknown_flags;
    u32 minimum_signatures;
    u32 maximum_signatures;
    u32 deadline;
    u32 issue_date;
    u32 allowed_guild_id;
    u32 allowed_classes;
    u32 allowed_races;
    u16 allowed_genders;
    u32 allowed_minimum_level;
    u32 allowed_maximum_level;
    u32 todo_amount_of_signers;
    u32 number_of_choices;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | petition_guid |  |  |
| 0x0C | 8 / Little | [Guid](../spec/packed-guid.md) | charter_owner |  |  |
| 0x14 | - / - | CString | guild_name |  |  |
| - | - / - | CString | body_text |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 4 / Little | u32 | unknown_flags |  | cmangos/vmangos/mangoszero: Set to 1, only info is comment from vmangos |
| - | 4 / Little | u32 | minimum_signatures |  | cmangos/vmangos/mangoszero: Set to 9, only info is comment from vmangos |
| - | 4 / Little | u32 | maximum_signatures |  | cmangos/vmangos/mangoszero: Set to 9, only info is comment from vmangos |
| - | 4 / Little | u32 | deadline |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 4 / Little | u32 | issue_date |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 4 / Little | u32 | allowed_guild_id |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 4 / Little | u32 | allowed_classes |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 4 / Little | u32 | allowed_races |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 2 / Little | u16 | allowed_genders |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 4 / Little | u32 | allowed_minimum_level |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 4 / Little | u32 | allowed_maximum_level |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 4 / Little | u32 | todo_amount_of_signers |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |
| - | 4 / Little | u32 | number_of_choices |  | cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos |

