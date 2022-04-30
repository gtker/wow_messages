## Client Version 1.12

```rust,ignore
smsg SMSG_TEXT_EMOTE = 0x0105 {
    Guid guid;    
    u32 text_emote;    
    Emote emote;    
    u32 name_length;    
    CString name;    
}

```
