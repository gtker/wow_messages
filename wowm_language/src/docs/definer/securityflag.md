## Protocol Version 3

```rust,ignore
enum SecurityFlag : u8 {
    NONE = 0x0;    
    PIN = 0x1;    
}

```
## Protocol Version 8

```rust,ignore
flag SecurityFlag : u8 {
    NONE = 0x00;    
    PIN = 0x01;    
    UNKNOWN0 = 0x02;    
    AUTHENTICATOR = 0x04;    
}

```
