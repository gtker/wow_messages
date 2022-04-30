## Client Version 1.12

### Comment

vmangos/cmangos/mangoszero: Seems to be older versions used to be 'amount_of_carriers' followed by array. All versions now just set first to 0 and have second be 0/1/2.
vmangos/cmangos/mangoszero: For AB and AV always set to all zero.

### Wowm Representation
```rust,ignore
smsg MSG_BATTLEGROUND_PLAYER_POSITIONS_Server = 0x02E9 {
    u32 amount_of_carriers = 0;    
    u32 amount_of_flag_carriers;    
    BattlegroundPlayerPosition[amount_of_flag_carriers] flag_carriers;    
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
| 0x04 | 4 / Little | u32 | amount_of_carriers |  |
| 0x08 | 4 / Little | u32 | amount_of_flag_carriers |  |
| 0x0C | ? / - | BattlegroundPlayerPosition[amount_of_flag_carriers] | flag_carriers |  |
