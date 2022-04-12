## Client Version 1.12

```rust,ignore
enum ActivateTaxiReply : u32 {
    OK = 0;    
    UNSPECIFIEDSERVERERROR = 1;    
    NOSUCHPATH = 2;    
    NOTENOUGHMONEY = 3;    
    TOOFARAWAY = 4;    
    NOVENDORNEARBY = 5;    
    NOTVISITED = 6;    
    PLAYERBUSY = 7;    
    PLAYERALREADYMOUNTED = 8;    
    PLAYERSHAPESHIFTED = 9;    
    PLAYERMOVING = 10;    
    SAMENODE = 11;    
    NOTSTANDING = 12;    
}

```
