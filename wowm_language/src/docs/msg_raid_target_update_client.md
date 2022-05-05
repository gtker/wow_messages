# MSG_RAID_TARGET_UPDATE_Client
## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg MSG_RAID_TARGET_UPDATE_Client = 0x0321 {
    RaidTargetIndex index;
    if (index != REQUEST_ICONS) {
        Guid target;
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
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x06 | ? / - | [RaidTargetIndex](raidtargetindex.md) | index |  |

If index is not equal to `REQUEST_ICONS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | target |  |
