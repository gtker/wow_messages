## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GMTICKET_GETTICKET = 0x0212 {
    GmTicketStatus status;
    if (status == HASTEXT) {
        CString text;
        GmTicketType ticket_type;
        f32 days_since_ticket_creation;
        f32 days_since_oldest_ticket_creation;
        f32 days_since_last_updated;
        GmTicketEscalationStatus escalation_status;
        u8 read_by_gm;
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
| 0x04 | ? / - | [GmTicketStatus](gmticketstatus.md) | status |  |  |

If status is equal to `HASTEXT`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | CString | text |  | cmangos: Ticket text: data, should never exceed 1999 bytes |
| - | ? / - | [GmTicketType](gmtickettype.md) | ticket_type |  |  |
| - | 4 / Little | f32 | days_since_ticket_creation |  |  |
| - | 4 / Little | f32 | days_since_oldest_ticket_creation |  |  |
| - | 4 / Little | f32 | days_since_last_updated |  |  |
| - | ? / - | [GmTicketEscalationStatus](gmticketescalationstatus.md) | escalation_status |  |  |
| - | 1 / - | u8 | read_by_gm |  |  |

