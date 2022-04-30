## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_BATTLEFIELD_STATUS = 0x02D4 {
    u32 queue_slot;    
    Map map;    
    if (map != EASTERN_KINGDOMS) {        
        u8 unknown0;        
        u32 client_instance_id;        
        StatusId status_id;        
        if (status_id == WAIT_QUEUE) {            
            u32 average_wait_time_in_ms;            
            u32 time_in_queue_in_ms;            
        }        
        else if (status_id == WAIT_JOIN) {            
            u32 time_to_remove_in_queue_in_ms;            
        }        
        else if (status_id == IN_PROGRESS) {            
            u32 time_to_bg_autoleave_in_ms;            
            u32 time_to_bg_start_in_ms;            
        }        
    }    
}

```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
