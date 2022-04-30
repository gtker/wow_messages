## Client Version 1.12

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
