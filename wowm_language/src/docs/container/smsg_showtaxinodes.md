## Client Version 1.12

```rust,ignore
smsg SMSG_SHOWTAXINODES = 0x1A9 {
    u32 unknown1;    
    Guid guid;    
    u32 nearest_node;    
    u32[-] nodes;    
}

```
