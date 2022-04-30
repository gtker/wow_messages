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
## Type
The basic type is `u8`, a 1 byte (8 bit) integer.
## Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `JOINED_NOTICE` | 0 (0x00) |  | %s joined channel. |
| `LEFT_NOTICE` | 1 (0x01) |  | %s left channel. |
| `YOU_JOINED_NOTICE` | 2 (0x02) |  | Joined Channel: [%s] -- You joined |
| `YOU_LEFT_NOTICE` | 3 (0x03) |  | Left Channel: [%s] -- You left |
| `WRONG_PASSWORD_NOTICE` | 4 (0x04) |  | Wrong password for %s. |
| `NOT_MEMBER_NOTICE` | 5 (0x05) |  | Not on channel %s. |
| `NOT_MODERATOR_NOTICE` | 6 (0x06) |  | Not a moderator of %s. |
| `PASSWORD_CHANGED_NOTICE` | 7 (0x07) |  | [%s] Password changed by %s. |
| `OWNER_CHANGED_NOTICE` | 8 (0x08) |  | [%s] Owner changed to %s. |
| `PLAYER_NOT_FOUND_NOTICE` | 9 (0x09) |  | [%s] Player %s was not found. |
| `NOT_OWNER_NOTICE` | 10 (0x0A) |  | [%s] You are not the channel owner. |
| `CHANNEL_OWNER_NOTICE` | 11 (0x0B) |  | [%s] Channel owner is %s. |
| `MODE_CHANGE_NOTICE` | 12 (0x0C) |  |  |
| `ANNOUNCEMENTS_ON_NOTICE` | 13 (0x0D) |  | [%s] Channel announcements enabled by %s. |
| `ANNOUNCEMENTS_OFF_NOTICE` | 14 (0x0E) |  | [%s] Channel announcements disabled by %s. |
| `MODERATION_ON_NOTICE` | 15 (0x0F) |  | [%s] Channel moderation enabled by %s. |
| `MODERATION_OFF_NOTICE` | 16 (0x10) |  | [%s] Channel moderation disabled by %s. |
| `MUTED_NOTICE` | 17 (0x11) |  | [%s] You do not have permission to speak. |
| `PLAYER_KICKED_NOTICE` | 18 (0x12) |  | [%s] Player %s kicked by %s. |
| `BANNED_NOTICE` | 19 (0x13) |  | [%s] You are banned from that channel. |
| `PLAYER_BANNED_NOTICE` | 20 (0x14) |  | [%s] Player %s banned by %s. |
| `PLAYER_UNBANNED_NOTICE` | 21 (0x15) |  | [%s] Player %s unbanned by %s. |
| `PLAYER_NOT_BANNED_NOTICE` | 22 (0x16) |  | [%s] Player %s is not banned. |
| `PLAYER_ALREADY_MEMBER_NOTICE` | 23 (0x17) |  | [%s] Player %s is already on the channel. |
| `INVITE_NOTICE` | 24 (0x18) |  | %2$s has invited you to join the channel '%1$s'. |
| `INVITE_WRONG_FACTION_NOTICE` | 25 (0x19) |  | Target is in the wrong alliance for %s. |
| `WRONG_FACTION_NOTICE` | 26 (0x1A) |  | Wrong alliance for %s. |
| `INVALID_NAME_NOTICE` | 27 (0x1B) |  | Invalid channel name |
| `NOT_MODERATED_NOTICE` | 28 (0x1C) |  | %s is not moderated |
| `PLAYER_INVITED_NOTICE` | 29 (0x1D) |  | [%s] You invited %s to join the channel |
| `PLAYER_INVITE_BANNED_NOTICE` | 30 (0x1E) |  | [%s] %s has been banned. |
| `THROTTLED_NOTICE` | 31 (0x1F) |  | [%s] The number of messages that can be sent to this channel is limited, please wait to send another message. |
