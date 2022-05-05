# SMSG_GUILD_INFO
## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GUILD_INFO = 0x0088 {
    CString guild_name;
    u32 created_day;
    u32 created_month;
    u32 created_year;
    u32 amount_of_characters_in_guild;
    u32 amount_of_accounts_in_guild;
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
| 0x04 | - / - | CString | guild_name |  |
| - | 4 / Little | u32 | created_day |  |
| - | 4 / Little | u32 | created_month |  |
| - | 4 / Little | u32 | created_year |  |
| - | 4 / Little | u32 | amount_of_characters_in_guild |  |
| - | 4 / Little | u32 | amount_of_accounts_in_guild |  |
