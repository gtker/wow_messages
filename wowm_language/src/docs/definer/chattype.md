## Client Version 1.12

## Wowm Representation
```rust,ignore
enum ChatType : u8 {
    SAY = 0x00;    
    PARTY = 0x01;    
    RAID = 0x02;    
    GUILD = 0x03;    
    OFFICER = 0x04;    
    YELL = 0x05;    
    WHISPER = 0x06;    
    WHISPER_INFORM = 0x07;    
    EMOTE = 0x08;    
    TEXT_EMOTE = 0x09;    
    SYSTEM = 0x0A;    
    MONSTER_SAY = 0x0B;    
    MONSTER_YELL = 0x0C;    
    MONSTER_EMOTE = 0x0D;    
    CHANNEL = 0x0E;    
    CHANNEL_JOIN = 0x0F;    
    CHANNEL_LEAVE = 0x10;    
    CHANNEL_LIST = 0x11;    
    CHANNEL_NOTICE = 0x12;    
    CHANNEL_NOTICE_USER = 0x13;    
    AFK = 0x14;    
    DND = 0x15;    
    IGNORED = 0x16;    
    SKILL = 0x17;    
    LOOT = 0x18;    
    MONSTER_WHISPER = 0x1A;    
    BG_SYSTEM_NEUTRAL = 0x52;    
    BG_SYSTEM_ALLIANCE = 0x53;    
    BG_SYSTEM_HORDE = 0x54;    
    RAID_LEADER = 0x57;    
    RAID_WARNING = 0x58;    
    RAID_BOSS_WHISPER = 0x59;    
    RAID_BOSS_EMOTE = 0x5A;    
    BATTLEGROUND = 0x5C;    
    BATTLEGROUND_LEADER = 0x5D;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| SAY | 0x00 | 0 | 0x0 |  |  |
| PARTY | 0x01 | 1 | 0x1 |  |  |
| RAID | 0x02 | 2 | 0x2 |  |  |
| GUILD | 0x03 | 3 | 0x3 |  |  |
| OFFICER | 0x04 | 4 | 0x4 |  |  |
| YELL | 0x05 | 5 | 0x5 |  |  |
| WHISPER | 0x06 | 6 | 0x6 |  |  |
| WHISPER_INFORM | 0x07 | 7 | 0x7 |  |  |
| EMOTE | 0x08 | 8 | 0x8 |  |  |
| TEXT_EMOTE | 0x09 | 9 | 0x9 |  |  |
| SYSTEM | 0x0A | 10 | 0xA |  |  |
| MONSTER_SAY | 0x0B | 11 | 0xB |  |  |
| MONSTER_YELL | 0x0C | 12 | 0xC |  |  |
| MONSTER_EMOTE | 0x0D | 13 | 0xD |  |  |
| CHANNEL | 0x0E | 14 | 0xE |  |  |
| CHANNEL_JOIN | 0x0F | 15 | 0xF |  |  |
| CHANNEL_LEAVE | 0x10 | 16 | 0x10 |  |  |
| CHANNEL_LIST | 0x11 | 17 | 0x11 |  |  |
| CHANNEL_NOTICE | 0x12 | 18 | 0x12 |  |  |
| CHANNEL_NOTICE_USER | 0x13 | 19 | 0x13 |  |  |
| AFK | 0x14 | 20 | 0x14 |  |  |
| DND | 0x15 | 21 | 0x15 |  |  |
| IGNORED | 0x16 | 22 | 0x16 |  |  |
| SKILL | 0x17 | 23 | 0x17 |  |  |
| LOOT | 0x18 | 24 | 0x18 |  |  |
| MONSTER_WHISPER | 0x1A | 26 | 0x1A |  |  |
| BG_SYSTEM_NEUTRAL | 0x52 | 82 | 0x52 |  |  |
| BG_SYSTEM_ALLIANCE | 0x53 | 83 | 0x53 |  |  |
| BG_SYSTEM_HORDE | 0x54 | 84 | 0x54 |  |  |
| RAID_LEADER | 0x57 | 87 | 0x57 |  |  |
| RAID_WARNING | 0x58 | 88 | 0x58 |  |  |
| RAID_BOSS_WHISPER | 0x59 | 89 | 0x59 |  |  |
| RAID_BOSS_EMOTE | 0x5A | 90 | 0x5A |  |  |
| BATTLEGROUND | 0x5C | 92 | 0x5C |  |  |
| BATTLEGROUND_LEADER | 0x5D | 93 | 0x5D |  |  |
