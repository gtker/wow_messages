## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_MESSAGECHAT = 0x0096 {
    ChatType chat_type;    
    Language language;    
    if (chat_type == MONSTER_WHISPER        
        || chat_type == RAID_BOSS_EMOTE        
        || chat_type == MONSTER_EMOTE) {        
        u32 name_length;        
        CString monster_name;        
        Guid monster_guid;        
    }    
    else if (chat_type == SAY        
        || chat_type == PARTY        
        || chat_type == YELL) {        
        Guid sender_guid1;        
        Guid sender_guid2;        
    }    
    else if (chat_type == MONSTER_SAY        
        || chat_type == MONSTER_YELL) {        
        Guid sender_guid3;        
        u32 sender_name_length;        
        CString sender_name;        
        Guid target_guid;        
    }    
    else if (chat_type == CHANNEL) {        
        CString channel_name;        
        u32 player_rank;        
        Guid player_guid;        
    }    
    else {    
        Guid sender_guid4;        
    }    
    u32 message_length;    
    CString message;    
    PlayerChatTag tag;    
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
| 0x04 | ? / - | ChatType | chat_type |  |
| - | ? / - | Language | language |  |

If chat_type is equal to `MONSTER_WHISPER` **or** 
is equal to `RAID_BOSS_EMOTE` **or** 
is equal to `MONSTER_EMOTE`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | name_length |  |
| - | - / - | CString | monster_name |  |
| - | 8 / Little | Guid | monster_guid |  |

Else If chat_type is equal to `SAY` **or** 
is equal to `PARTY` **or** 
is equal to `YELL`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 8 / Little | Guid | sender_guid1 |  |
| - | 8 / Little | Guid | sender_guid2 |  |

Else If chat_type is equal to `MONSTER_SAY` **or** 
is equal to `MONSTER_YELL`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 8 / Little | Guid | sender_guid3 |  |
| - | 4 / Little | u32 | sender_name_length |  |
| - | - / - | CString | sender_name |  |
| - | 8 / Little | Guid | target_guid |  |

Else If chat_type is equal to `CHANNEL`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | - / - | CString | channel_name |  |
| - | 4 / Little | u32 | player_rank |  |
| - | 8 / Little | Guid | player_guid |  |

Else: 
| - | 8 / Little | Guid | sender_guid4 |  |
| - | 4 / Little | u32 | message_length |  |
| - | - / - | CString | message |  |
| - | ? / - | PlayerChatTag | tag |  |
