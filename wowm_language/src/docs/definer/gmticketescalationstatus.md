## Client Version 1.12

### Wowm Representation
```rust,ignore
enum GmTicketEscalationStatus : u8 {
    GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED = 0;
    GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED = 1;
    GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED = 2;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED` | 0 (0x00) |  | ticket is not currently assigned to a gm |
| `GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED` | 1 (0x01) |  | ticket is assigned to a normal gm |
| `GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED` | 2 (0x02) |  | ticket is in the escalation queue |
