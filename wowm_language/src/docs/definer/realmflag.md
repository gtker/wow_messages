## Protocol Version 2, Protocol Version 3

## Wowm Representation
```rust,ignore
flag RealmFlag : u8 {
    NONE = 0x00;    
    INVALID = 0x01;    
    OFFLINE = 0x02;    
    FORCE_BLUE_RECOMMENDED = 0x20;    
    FORCE_GREEN_RECOMMENDED = 0x40;    
    FORCE_RED_FULL = 0x80;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0x00 | 0 | 0x0 |  |  |
| INVALID | 0x01 | 1 | 0x1 |  |  |
| OFFLINE | 0x02 | 2 | 0x2 |  |  |
| FORCE_BLUE_RECOMMENDED | 0x20 | 32 | 0x20 |  |  |
| FORCE_GREEN_RECOMMENDED | 0x40 | 64 | 0x40 |  |  |
| FORCE_RED_FULL | 0x80 | 128 | 0x80 |  |  |
## Protocol Version 8

## Wowm Representation
```rust,ignore
flag RealmFlag : u8 {
    NONE = 0x00;    
    INVALID = 0x01;    
    OFFLINE = 0x02;    
    SPECIFY_BUILD = 0x04;    
    FORCE_BLUE_RECOMMENDED = 0x20;    
    FORCE_GREEN_RECOMMENDED = 0x40;    
    FORCE_RED_FULL = 0x80;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0x00 | 0 | 0x0 |  |  |
| INVALID | 0x01 | 1 | 0x1 |  |  |
| OFFLINE | 0x02 | 2 | 0x2 |  |  |
| SPECIFY_BUILD | 0x04 | 4 | 0x4 |  |  |
| FORCE_BLUE_RECOMMENDED | 0x20 | 32 | 0x20 |  |  |
| FORCE_GREEN_RECOMMENDED | 0x40 | 64 | 0x40 |  |  |
| FORCE_RED_FULL | 0x80 | 128 | 0x80 |  |  |
