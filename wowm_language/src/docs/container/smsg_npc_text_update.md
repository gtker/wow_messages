## Client Version 1.12

```rust,ignore
smsg SMSG_NPC_TEXT_UPDATE = 0x0180 {
    u32 text_id;    
    f32 probability;    
    NpcTextUpdate[8] texts;    
}

```
