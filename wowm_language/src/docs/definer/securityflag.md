## Protocol Version 3

## Wowm Representation
```rust,ignore
enum SecurityFlag : u8 {
    NONE = 0x0;    
    PIN = 0x1;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0x0 | 0 | 0x0 |  |  |
| PIN | 0x1 | 1 | 0x1 |  |  |
## Protocol Version 8

## Wowm Representation
```rust,ignore
flag SecurityFlag : u8 {
    NONE = 0x00;    
    PIN = 0x01;    
    UNKNOWN0 = 0x02;    
    AUTHENTICATOR = 0x04;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0x00 | 0 | 0x0 |  |  |
| PIN | 0x01 | 1 | 0x1 |  |  |
| UNKNOWN0 | 0x02 | 2 | 0x2 |  |  |
| AUTHENTICATOR | 0x04 | 4 | 0x4 |  |  |
