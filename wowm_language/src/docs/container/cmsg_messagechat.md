## Client Version 1.12

```rust,ignore
cmsg CMSG_MESSAGECHAT = 0x95 {
    ChatType chat_type;    
    Language language;    
    if (chat_type == SAY        
        || chat_type == EMOTE        
        || chat_type == YELL        
        || chat_type == PARTY        
        || chat_type == GUILD        
        || chat_type == OFFICER        
        || chat_type == RAID        
        || chat_type == RAID_LEADER        
        || chat_type == RAID_WARNING        
        || chat_type == BATTLEGROUND        
        || chat_type == BATTLEGROUND_LEADER        
        || chat_type == AFK        
        || chat_type == DND) {        
        CString message;        
    }    
    else if (chat_type == WHISPER) {        
        CString target_player;        
        CString whisper_message;        
    }    
    else if (chat_type == CHANNEL) {        
        CString channel;        
        CString channel_message;        
    }    
}

```
