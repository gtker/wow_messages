## Protocol Version *

### Wowm Representation
```rust,ignore
enum Locale : u32 {
    EN_GB = "enGB";
    EN_US = "enUS";
    ES_MX = "esMX";
    PT_BR = "ptBR";
    FR_FR = "frFR";
    DE_DE = "deDE";
    ES_ES = "esES";
    PT_PT = "ptPT";
    IT_IT = "itIT";
    RU_RU = "ruRU";
    KO_KR = "koKR";
    ZH_TW = "zhTW";
    EN_TW = "enTW";
    EN_CN = "enCN";
    OTHER = self.value
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `EN_GB` | 1701726018 (0x656E4742) |  |  |
| `EN_US` | 1701729619 (0x656E5553) |  |  |
| `ES_MX` | 1702055256 (0x65734D58) |  |  |
| `PT_BR` | 1886667346 (0x70744252) |  |  |
| `FR_FR` | 1718765138 (0x66724652) |  |  |
| `DE_DE` | 1684358213 (0x64654445) |  |  |
| `ES_ES` | 1702053203 (0x65734553) |  |  |
| `PT_PT` | 1886670932 (0x70745054) |  |  |
| `IT_IT` | 1769228628 (0x69744954) |  |  |
| `RU_RU` | 1920291413 (0x72755255) |  |  |
| `KO_KR` | 1802455890 (0x6B6F4B52) |  |  |
| `ZH_TW` | 2053657687 (0x7A685457) |  |  |
| `EN_TW` | 1701729367 (0x656E5457) |  |  |
| `EN_CN` | 1701725006 (0x656E434E) |  |  |
