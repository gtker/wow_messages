# UpdateMask

Object
    - ItemFields
    - ContainerFields

    - UnitFields
    - PlayerFields

    - GameObjectFields
    - DynamicObjectFields
    - CorpseFields

```rust,ignore
struct ObjectFields {
    guid: Option<u64>,
    ty: Option<u32>,
    entry: Option<u32>,
    scale_x: Option<f32>,
}

struct UpdateMask {
    pub guid: Option<u64>,

    pub owner: Option<u64>,
}

impl UpdateMask {
    
}
```
