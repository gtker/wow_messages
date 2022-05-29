## Client Version 1.2, Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_AUTH_RESPONSE = 0x01EE {
    WorldResult result;
    if (result == AUTH_OK) {
        u32 billing_time;
        u8 billing_flags;
        u32 billing_rested;
    }
    else if (result == AUTH_WAIT_QUEUE) {
        u32 queue_position;
    }
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
| 0x04 | ? / - | [WorldResult](worldresult.md) | result |  |  |

If result is equal to `AUTH_OK`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | billing_time |  |  |
| - | 1 / - | u8 | billing_flags |  |  |
| - | 4 / Little | u32 | billing_rested |  |  |

Else If result is equal to `AUTH_WAIT_QUEUE`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | queue_position |  |  |

