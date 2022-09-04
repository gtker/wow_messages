# Containers

All containers have:

* `name`, type name of the container.
* `object_type` contains:
    * `type`: either `struct`, `clogin`, `slogin`, `cmsg`, `smsg`, or `msg`.
    * `opcode`, only present if `type` is not struct.
* `constant`, `true` if all fields always have the same size, otherwise `false`.
* `members`, an array of Container Members.
* `tags`, a [Tags](tags.md) object.
* `tests`, an array of Container Tests.
* `file_info`, a [FileInfo](fileinfo.md) object.
* `only_has_io_error`, `true` if the contain does not contain an enum (which can error by having a non-existing enumerator value), a String or CString (which can contain invalid UTF-8 values) or a struct that has either, otherwise `false`.
* `features`, an array of [Features](features.md) required.

## Member

The member array is a representation of the layout of the container.

It contains:

* `type`, either `definition`, `if_statement`, or `optional`.
* `content`, the values that belong to a specific `type`.


### Definition

Definitions represent a specific value that is present in the container.
They contain:

* `name`, the variable name of the definition.
* `member_type`, the type of the definition, contains:
    * `type`, either `integer`, `packed_guid`, `guid`, `floating_point`, `cstring`, `sized_cstring`, `string`, `array`, `identifier`, `update_mask`, or `aura_mask`.
    * `content`, depends on the type.
* `constant_value`, `null` if the field is not always supposed to have the same value. Otherwise it contains:
    * `value`, the constant value it should always be represented as.
    * `original_string`, the original string that lead to the `value`. This can also be `self.size` to represent that the amount of remaining bytes in the message should be input instead of `value`.
* `used_as_size_in`, the field in the current container that the value of this field should be used as the size of, only for integer values. `null` if not used as a size.
* `used_in_if`, `true` if the definition is used in an `if_statement`, otherwise `false`.
* `tags`, a [Tags](tags.md) object.


#### Type Content

The following types do not contain any `content`:

* `packed_guid`
* `guid`
* `cstring`
* `sized_cstring`
* `update_mask`
* `aura_mask`


##### Integer

The `integer` type contains a `content` with:
* `type`, can be any of `u8`, `u16`, `u32` or `u64`.
* `endianness`, either `little` or `big`. Not present when `type` is `u8`.

##### Floating Point

The `floating_point` type contains a `content` with:
* `type`, either `f32` (float) or `f64` (double).
* `endianness`, either `little` or `big`.

##### String

The `string` type contains a `content` with:
* `length`, the variable name of the definition used to signify the length of the string.

##### Array

The `array` type contains a `content` with:
* `inner`, representing the inner type of the array. This contains the fields:
    * `type`, which is either `integer`, `complex`, `cstring`, `guid` or `packed_guid`.
    * `content`, which depends on the `type`.
        * `integer` has the fields:
            * `type`, can be any of `u8`, `u16`, `u32` or `u64`.
            * `endianness`, either `little` or `big`. Not present when `type` is `u8`.
        * `complex` has no fields, but the content is a string with the type name used in the array.
        * `cstring`, `guid`, and `packed_guid` do not have any `content`.
* `size`, representing how the size of the array is known. This contains the fields:
    * `type`, which is either `fixed`, `variable`, or `endless`.
    * `content`, which is:
        * The amount of array members if the size is `fixed`.
        * The variable name that determines amount of members if `variable`.
        * Not present if `endless`. Endless arrays take up all remaining bytes in the message.

##### Identifier

The `identifier` type contains a `content` with:
* `type_name`, the type name of the variable.
* `upcast`, contains the same values as an `integer` `type` field. Only present if the type is an enum where the type in the message is larger than the native type of the definer.

### If Statement

### Optional



## Test
