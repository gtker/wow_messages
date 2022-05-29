## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GMTICKET_SYSTEMSTATUS = 0x021B {
    u32 will_accept_tickets;
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
| 0x04 | 4 / Little | u32 | will_accept_tickets |  | mangoszero/cmangos/vmangos all only send 1 for true and 0 for false. vmangos: Note: This only disables the ticket UI at client side and is not fully reliable are we sure this is a uint32? Should ask Zor |

