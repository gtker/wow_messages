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
