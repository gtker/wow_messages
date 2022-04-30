## Client Version 1.12

```rust,ignore
smsg SMSG_PARTY_COMMAND_RESULT = 0x7F {
    PartyOperation operation;    
    CString member;    
    PartyResult result;    
}

```
