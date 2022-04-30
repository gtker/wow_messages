## Client Version 1.12

## Wowm Representation
```rust,ignore
enum ChatNotify : u8 {
    JOINED_NOTICE = 0x00;    
    LEFT_NOTICE = 0x01;    
    YOU_JOINED_NOTICE = 0x02;    
    YOU_LEFT_NOTICE = 0x03;    
    WRONG_PASSWORD_NOTICE = 0x04;    
    NOT_MEMBER_NOTICE = 0x05;    
    NOT_MODERATOR_NOTICE = 0x06;    
    PASSWORD_CHANGED_NOTICE = 0x07;    
    OWNER_CHANGED_NOTICE = 0x08;    
    PLAYER_NOT_FOUND_NOTICE = 0x09;    
    NOT_OWNER_NOTICE = 0x0A;    
    CHANNEL_OWNER_NOTICE = 0x0B;    
    MODE_CHANGE_NOTICE = 0x0C;    
    ANNOUNCEMENTS_ON_NOTICE = 0x0D;    
    ANNOUNCEMENTS_OFF_NOTICE = 0x0E;    
    MODERATION_ON_NOTICE = 0x0F;    
    MODERATION_OFF_NOTICE = 0x10;    
    MUTED_NOTICE = 0x11;    
    PLAYER_KICKED_NOTICE = 0x12;    
    BANNED_NOTICE = 0x13;    
    PLAYER_BANNED_NOTICE = 0x14;    
    PLAYER_UNBANNED_NOTICE = 0x15;    
    PLAYER_NOT_BANNED_NOTICE = 0x16;    
    PLAYER_ALREADY_MEMBER_NOTICE = 0x17;    
    INVITE_NOTICE = 0x18;    
    INVITE_WRONG_FACTION_NOTICE = 0x19;    
    WRONG_FACTION_NOTICE = 0x1A;    
    INVALID_NAME_NOTICE = 0x1B;    
    NOT_MODERATED_NOTICE = 0x1C;    
    PLAYER_INVITED_NOTICE = 0x1D;    
    PLAYER_INVITE_BANNED_NOTICE = 0x1E;    
    THROTTLED_NOTICE = 0x1F;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| JOINED_NOTICE | 0x00 | 0 | 0x0 |  | %s joined channel. |
| LEFT_NOTICE | 0x01 | 1 | 0x1 |  | %s left channel. |
| YOU_JOINED_NOTICE | 0x02 | 2 | 0x2 |  | Joined Channel: [%s] -- You joined |
| YOU_LEFT_NOTICE | 0x03 | 3 | 0x3 |  | Left Channel: [%s] -- You left |
| WRONG_PASSWORD_NOTICE | 0x04 | 4 | 0x4 |  | Wrong password for %s. |
| NOT_MEMBER_NOTICE | 0x05 | 5 | 0x5 |  | Not on channel %s. |
| NOT_MODERATOR_NOTICE | 0x06 | 6 | 0x6 |  | Not a moderator of %s. |
| PASSWORD_CHANGED_NOTICE | 0x07 | 7 | 0x7 |  | [%s] Password changed by %s. |
| OWNER_CHANGED_NOTICE | 0x08 | 8 | 0x8 |  | [%s] Owner changed to %s. |
| PLAYER_NOT_FOUND_NOTICE | 0x09 | 9 | 0x9 |  | [%s] Player %s was not found. |
| NOT_OWNER_NOTICE | 0x0A | 10 | 0xA |  | [%s] You are not the channel owner. |
| CHANNEL_OWNER_NOTICE | 0x0B | 11 | 0xB |  | [%s] Channel owner is %s. |
| MODE_CHANGE_NOTICE | 0x0C | 12 | 0xC |  |  |
| ANNOUNCEMENTS_ON_NOTICE | 0x0D | 13 | 0xD |  | [%s] Channel announcements enabled by %s. |
| ANNOUNCEMENTS_OFF_NOTICE | 0x0E | 14 | 0xE |  | [%s] Channel announcements disabled by %s. |
| MODERATION_ON_NOTICE | 0x0F | 15 | 0xF |  | [%s] Channel moderation enabled by %s. |
| MODERATION_OFF_NOTICE | 0x10 | 16 | 0x10 |  | [%s] Channel moderation disabled by %s. |
| MUTED_NOTICE | 0x11 | 17 | 0x11 |  | [%s] You do not have permission to speak. |
| PLAYER_KICKED_NOTICE | 0x12 | 18 | 0x12 |  | [%s] Player %s kicked by %s. |
| BANNED_NOTICE | 0x13 | 19 | 0x13 |  | [%s] You are banned from that channel. |
| PLAYER_BANNED_NOTICE | 0x14 | 20 | 0x14 |  | [%s] Player %s banned by %s. |
| PLAYER_UNBANNED_NOTICE | 0x15 | 21 | 0x15 |  | [%s] Player %s unbanned by %s. |
| PLAYER_NOT_BANNED_NOTICE | 0x16 | 22 | 0x16 |  | [%s] Player %s is not banned. |
| PLAYER_ALREADY_MEMBER_NOTICE | 0x17 | 23 | 0x17 |  | [%s] Player %s is already on the channel. |
| INVITE_NOTICE | 0x18 | 24 | 0x18 |  | %2$s has invited you to join the channel '%1$s'. |
| INVITE_WRONG_FACTION_NOTICE | 0x19 | 25 | 0x19 |  | Target is in the wrong alliance for %s. |
| WRONG_FACTION_NOTICE | 0x1A | 26 | 0x1A |  | Wrong alliance for %s. |
| INVALID_NAME_NOTICE | 0x1B | 27 | 0x1B |  | Invalid channel name |
| NOT_MODERATED_NOTICE | 0x1C | 28 | 0x1C |  | %s is not moderated |
| PLAYER_INVITED_NOTICE | 0x1D | 29 | 0x1D |  | [%s] You invited %s to join the channel |
| PLAYER_INVITE_BANNED_NOTICE | 0x1E | 30 | 0x1E |  | [%s] %s has been banned. |
| THROTTLED_NOTICE | 0x1F | 31 | 0x1F |  | [%s] The number of messages that can be sent to this channel is limited, please wait to send another message. |
