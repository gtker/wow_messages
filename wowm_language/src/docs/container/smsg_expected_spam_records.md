## Client Version 1.12

```rust,ignore
smsg SMSG_EXPECTED_SPAM_RECORDS = 0x332 {
    u32 amount_of_records;    
    CString[amount_of_records] records;    
}

```
