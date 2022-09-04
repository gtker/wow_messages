# Definers

All definers have:

* `name`, the type name of the Definer.
* `definer_ty`, either `flag` or `enum`.
* `enumerators`, an array of Enumerators.
* `self_value`, only for `Enums`, a specialized enumerator that should cover all remaining values that do not have a specific enumerator, contains:
    * `name`
    * `tags`
* `integer_type`, the underlying integer type representation, contains:
    * `type`, can be any of `u8`, `u16`, `u32` or `u64`.
    * `endianness`, either `little` or `big`. Not present when `type` is `u8`.
* `tags`, a [Tags](tags.md) object.
* `objects_used_in`, an array of objects with:
    * `name`, name of the object.
    * `usage`, either `used_but_not_in_if` or `in_if`. 
* `file_info`, a [FileInfo](fileinfo.md) object.
* `features`, an array of strings with required [Features](../spec/features.md).

A definer with two enumerators could look like:

```json
{
    "name": "Os",
        "definer_ty": "enum",
        "enumerators": [
            {
                "name": "WINDOWS",
                "value": {
                    "int": 5728622,
                    "original": "\"\\0Win\""
                },
                "tags": {}
            },
            {
                "name": "OSX",
                "value": {
                    "int": 5198680,
                    "original": "\"\\0OSX\""
                },
                "tags": {}
            }
        ],
        "self_value": {
            "name": "OTHER",
            "tags": {}
        },
        "integer_type": {
            "type": "u32",
            "endianness": "little"
        },
        "tags": {
            "version": {
                "type": "login",
                "versions": [
                {
                    "type": "all"
                }
                ]
            }
        },
        "objects_used_in": [
        {
            "name": "CMD_AUTH_LOGON_CHALLENGE_Client",
            "usage": "used_but_not_in_if"
        },
        {
            "name": "CMD_AUTH_RECONNECT_CHALLENGE_Client",
            "usage": "used_but_not_in_if"
        }
        ],
        "file_info": {
            "file_name": "wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm",
            "start_position": 3
        },
        "features": [
            "Enum",
        "EnumSelf",
        "LoginVersion"
        ]
},
```

## Enumerators

All enumerators have:

* `name`, the name of the enumerator.
* `value`, containing:
    * `int`, the integer value.
    * `original`, a string of the original value.
* `tags`, a [Tags](tags.md) object.
