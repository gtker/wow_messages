## Client Version 1.12

## Wowm Representation
```rust,ignore
enum GmTicketEscalationStatus : u8 {
    GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED = 0;    
    GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED = 1;    
    GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED = 2;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED | 0 | 0 | 0x0 |  | ticket is not currently assigned to a gm |
| GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED | 1 | 1 | 0x1 |  | ticket is assigned to a normal gm |
| GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED | 2 | 2 | 0x2 |  | ticket is in the escalation queue |
