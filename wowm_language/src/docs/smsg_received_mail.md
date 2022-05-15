## Client Version 1.12

### Comment

cmangos/vmangos/mangoszero: deliver undelivered mail

### Wowm Representation
```rust,ignore
smsg SMSG_RECEIVED_MAIL = 0x0285 {
    u32 unknown1;
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
| 0x04 | 4 / Little | u32 | unknown1 |  | cmangos/vmangos sends 0 as u32, mangoszero sends 0 as f32 |
