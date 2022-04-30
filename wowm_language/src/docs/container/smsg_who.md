## Client Version 1.12

```rust,ignore
smsg SMSG_WHO = 0x63 {
    u32 listed_players;    
    u32 online_players;    
    WhoPlayer[listed_players] players;    
}

```
