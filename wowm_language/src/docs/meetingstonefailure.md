## Client Version 1.12

### Wowm Representation
```rust,ignore
enum MeetingStoneFailure : u8 {
    MEETINGSTONE_FAIL_PARTYLEADER = 1;
    MEETINGSTONE_FAIL_FULL_GROUP = 2;
    MEETINGSTONE_FAIL_RAID_GROUP = 3;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `MEETINGSTONE_FAIL_PARTYLEADER` | 1 (0x01) |  |  |
| `MEETINGSTONE_FAIL_FULL_GROUP` | 2 (0x02) |  |  |
| `MEETINGSTONE_FAIL_RAID_GROUP` | 3 (0x03) |  |  |
