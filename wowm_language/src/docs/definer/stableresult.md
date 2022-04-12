## Client Version 1.12

```rust,ignore
enum StableResult : u8 {
    ERR_MONEY = 0x01;    
    ERR_STABLE = 0x06;    
    SUCCESS_STABLE = 0x08;    
    SUCCESS_UNSTABLE = 0x09;    
    SUCCESS_BUY_SLOT = 0x0A;    
}

```
