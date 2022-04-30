## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PARTY_COMMAND_RESULT = 0x007F {
    PartyOperation operation;    
    CString member;    
    PartyResult result;    
}

```
