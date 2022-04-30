## Client Version 1.12

## Wowm Representation
```rust,ignore
cmsg CMSG_AUTH_SESSION = 0x01ED {
    u32 build;    
    u32 server_id;    
    CString username;    
    u32 client_seed;    
    u8[20] client_proof;    
    u32 decompressed_addon_info_size;    
    u8[-] compressed_addon_info;    
}

```
