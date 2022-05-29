# SMSG_CREATURE_QUERY_RESPONSE

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_CREATURE_QUERY_RESPONSE = 0x0061 {
    u32 creature_entry;
    optional found {
        CString name1;
        CString name2;
        CString name3;
        CString name4;
        CString sub_name;
        u32 type_flags;
        u32 creature_type;
        u32 creature_family;
        u32 creature_rank;
        u32 unknown0;
        u32 spell_data_id;
        u32 display_id;
        u8 civilian;
        u8 racial_leader;
    }
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
| 0x04 | 4 / Little | u32 | creature_entry |  |  |

Optionally the following fields can be present. This can only be detected by looking at the size of the message.

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x08 | - / - | CString | name1 |  |  |
| - | - / - | CString | name2 |  |  |
| - | - / - | CString | name3 |  |  |
| - | - / - | CString | name4 |  |  |
| - | - / - | CString | sub_name |  |  |
| - | 4 / Little | u32 | type_flags |  |  |
| - | 4 / Little | u32 | creature_type |  | cmangos: CreatureType.dbc   wdbFeild8 |
| - | 4 / Little | u32 | creature_family |  | cmangos: CreatureFamily.dbc |
| - | 4 / Little | u32 | creature_rank |  | cmangos: Creature Rank (elite, boss, etc) |
| - | 4 / Little | u32 | unknown0 |  | cmangos: wdbFeild11 |
| - | 4 / Little | u32 | spell_data_id |  | cmangos: Id from CreatureSpellData.dbc wdbField12 |
| - | 4 / Little | u32 | display_id |  | cmangos: DisplayID      wdbFeild13 and workaround, way to manage models must be fixed |
| - | 1 / - | u8 | civilian |  | cmangos: wdbFeild14 |
| - | 1 / - | u8 | racial_leader |  |  |

