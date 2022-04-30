## Client Version 1.12

```rust,ignore
smsg SMSG_MAIL_LIST_RESULT = 0x23B {
    u8 amount_of_mails;    
    Mail[amount_of_mails] mails;    
}

```
