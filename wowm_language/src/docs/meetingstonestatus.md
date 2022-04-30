## Client Version 1.12

### Wowm Representation
```rust,ignore
enum MeetingStoneStatus : u8 {
    LEAVE_QUEUE = 0;
    JOINED_QUEUE = 1;
    PARTY_MEMBER_LEFT_LFG = 2;
    PARTY_MEMBER_REMOVED_PARTY_REMOVED = 3;
    LOOKING_FOR_NEW_PARTY_IN_QUEUE = 4;
    NONE = 5;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `LEAVE_QUEUE` | 0 (0x00) |  |  |
| `JOINED_QUEUE` | 1 (0x01) |  |  |
| `PARTY_MEMBER_LEFT_LFG` | 2 (0x02) |  |  |
| `PARTY_MEMBER_REMOVED_PARTY_REMOVED` | 3 (0x03) |  |  |
| `LOOKING_FOR_NEW_PARTY_IN_QUEUE` | 4 (0x04) |  |  |
| `NONE` | 5 (0x05) |  |  |
