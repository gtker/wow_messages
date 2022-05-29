## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg MSG_PVP_LOG_DATA_Server = 0x02E0 {
    BattlegroundEndStatus status;
    if (status == ENDED) {
        BattlegroundWinner winner;
    }
    u32 amount_of_players;
    BattlegroundPlayer[amount_of_players] players;
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
| 0x04 | ? / - | [BattlegroundEndStatus](battlegroundendstatus.md) | status |  |  |

If status is equal to `ENDED`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | [BattlegroundWinner](battlegroundwinner.md) | winner |  |  |
| - | 4 / Little | u32 | amount_of_players |  | vmangos: Client has a hard limit to 80. If we go beyond (but it should not happen ?!), WoW Error (happening !) |
| - | ? / - | [BattlegroundPlayer](battlegroundplayer.md)[amount_of_players] | players |  |  |

