## Client Version 1.12

```rust,ignore
smsg SMSG_LIST_INVENTORY = 0x19F {
    Guid vendor;    
    u8 amount_of_items;    
    ListInventoryItem[amount_of_items] items;    
}

```
