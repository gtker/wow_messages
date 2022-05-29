# SMSG_MEETINGSTONE_SETQUEUE

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_MEETINGSTONE_SETQUEUE = 0x0295 {
    Area area;
    MeetingStoneStatus status;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | ? / - | [Area](area.md) | area |  |  |
| - | ? / - | [MeetingStoneStatus](meetingstonestatus.md) | status |  |  |

