# Implementation Features

## Declarations

### `SIMPLE_BUILT_IN_TYPES`

```rust,ignore
struct BuiltInTypes {
    u8 b_u8;
    u16 b_u16;
    u32 b_u32;
    u64 b_u64;
    u16_be b_u16_be;
    u32_be b_u32_be;
    u64_be b_u64_be;
    f32 b_f32;
    f64 b_f64;
    f32_be b_f32_be;
    f64_be b_f64_be;
    CString c_string;
    String[10] fixed_string;
    String[b_u8] variable_string;
}
/* This also uses feature STRUCT */
```

### `SIMPLE_DEFINER_TYPES`

```rust,ignore
enum SimpleEnum : u8 {
    ZERO = 0;
    ONE = 1;
}
/* This also uses feature ENUM */

flag SimpleFlag : u8 {
    NONE = 0;
    SOME = 1;
}
/* This also uses feature FLAG */

struct SimpleDefinerTypes {
    SimpleEnum e;
    SimpleFlag f;

    (u16)SimpleEnum e_u16;
    (u16_be)SimpleEnum e_u16;

    (u32)SimpleEnum e_u32;
    (u32_be)SimpleEnum e_u32;

    (u64)SimpleEnum e_u64;
    (u64_be)SimpleEnum e_u64;
}
/* This also uses feature STRUCT */
```

### `PACKED_GUID_TYPES`

```rust,ignore
struct PackedGuidTypes {
    PackedGuid p;
}
/* This also uses feature STRUCT */
```

### `UPDATE_MASK_TYPES`

```rust,ignore
struct UpdateMaskTypes {
    UpdateMask u;
}
/* This also uses feature STRUCT */
```

### `AURA_MASK_TYPES`

```rust,ignore
struct AuraMaskTypes {
    AuraMask a;
}
/* This also uses feature STRUCT */
```

### `NESTED_STRUCT_TYPES`

```rust,ignore
struct InnerStruct {
    u8 b_u8;
}

struct NestedStructsTypes {
    InnerStruct is;
}
```

### `SIMPLE_ARRAYS`

```rust,ignore
struct SimpleArrays {
    u8 i_u8;
    u8[10] fixed_u8;
    u8[i_u8] variable_u8;

    u16 i_u16;
    u16[10] fixed_u16;
    u16[i_u16] variable_u16;
    u16_be i_u16_be;
    u16_be[10] fixed_u16_be;
    u16_be[i_u16_be] variable_u16_be;

    u32 i_u32;
    u32[10] fixed_u32;
    u32[i_u32] variable_u32;
    u32_be i_u32_be;
    u32_be[10] fixed_u32_be;
    u32_be[i_u32_be] variable_u32_be;

    u64 i_u64;
    u64[10] fixed_u64;
    u64[i_u64] variable_u64;
    u64_be i_u64_be;
    u64_be[10] fixed_u64_be;
    u64_be[i_u64_be] variable_u64_be;

    u8 i_f32;
    f32[10] fixed_f32;
    f32[i_f32] variable_f32;
    u8 i_f32_be;
    f32_be[10] fixed_f32_be;
    f32_be[i_f32_be] variable_f32_be;

    u8 i_f64;
    f64[10] fixed_f64;
    f64[i_f64] variable_f64;
    u8 i_f64_be;
    f64_be[10] fixed_f64_be;
    f64_be[i_f64_be] variable_f64_be;

    u8 i_cstring;
    CString[10] fixed_cstring;
    CString[i_cstring] variable_cstring;

    /* No String[] arrays */
}
/* This also uses feature STRUCT and SIMPLE_BUILT_IN_TYPES */
```

### `COMPLEX_ARRAYS`

```rust,ignore
flag SimpleFlag : u8 {
    NONE = 0;
    SOME = 1;
}

enum SimpleEnum : u8 {
    ZERO = 0;
    ONE = 1;
}

struct InnerStruct {
    u8 b_u8;
}

struct ComplexArrays {
    u8 i_struct;
    InnerStruct[10] fixed_struct;
    InnerStruct[i_struct] variable_struct;

    u8 i_flag;
    SimpleFlag[10] fixed_flag;
    SimpleFlag[i_flag] variable_flag;

    u8 i_enum;
    SimpleEnum[10] fixed_enum;
    SimpleEnum[i_enum] variable_enum;

    u8 i_packed_guid;
    PackedGuid[10] fixed_packed_guid;
    PackedGuid[i_packed_guid] variable_packed_guid;

    u8 i_update_mask;
    UpdateMask[10] fixed_update_mask;
    UpdateMask[i_update_mask] variable_update_mask;

    u8 i_aura_mask;
    AuraMask[10] fixed_aura_mask;
    AuraMask[i_aura_mask] variable_aura_mask;
}
/* This also uses feature STRUCT, SIMPLE_BUILT_IN_TYPES
    PACKED_GUID_TYPES, AURA_MASK_TYPES and UPDATE_MASK_TYPES, 
    COMPLEX_TYPES.
*/
```

## Objects

### Enums
#### `ENUM`

Any enumerator without a `self.value` field.

### `ENUM_SELF`

Any enumerator with a `self.value` field.

### Flags
#### `FLAG`

Any flag.

### Structs
#### `STRUCT`

Any struct.

### Login Messages
#### `CLOGIN`
#### `SLOGIN`

### World Messages
#### `SMSG`
#### `CMSG`
#### `MSG`

## Versions

### `WORLD_VERSION`

Any object with a world version (`versions` [tag](tags.md)).

### `LOGIN_VERSION`

Any object with a login version (`logon_login_versions` [tag](tags.md)).

