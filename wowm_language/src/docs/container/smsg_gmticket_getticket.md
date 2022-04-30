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
