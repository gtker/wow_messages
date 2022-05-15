## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_MESSAGECHAT = 0x0095 {
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
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | ? / - | [ChatType](chattype.md) | chat_type |  |  |
| - | ? / - | [Language](language.md) | language |  |  |

If chat_type is equal to `SAY` **or** 
is equal to `EMOTE` **or** 
is equal to `YELL` **or** 
is equal to `PARTY` **or** 
is equal to `GUILD` **or** 
is equal to `OFFICER` **or** 
is equal to `RAID` **or** 
is equal to `RAID_LEADER` **or** 
is equal to `RAID_WARNING` **or** 
is equal to `BATTLEGROUND` **or** 
is equal to `BATTLEGROUND_LEADER` **or** 
is equal to `AFK` **or** 
is equal to `DND`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | CString | message |  |  |

Else If chat_type is equal to `WHISPER`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | CString | target_player |  |  |
| - | - / - | CString | whisper_message |  |  |

Else If chat_type is equal to `CHANNEL`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | CString | channel |  |  |
| - | - / - | CString | channel_message |  |  |
