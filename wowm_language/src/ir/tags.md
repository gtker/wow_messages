# Tags

Tags give additional information to an object, enumerator or variable.

Tags can contain:

* `description`, a string describing the tagged item.
* `comment`, a string commenting on the tagged item.
* `display`, for enumerators only, a string with a pretty printed version of the name.
* `unimplemented`, a string containing `true`. If this is present the body of the message is not implemented and just contains an endless u8 array.
* `version`, contains:
    * `type`, either `login` for Login messages (CLogin, SLogin), or `world` for World messages (CMSG, SMSG, MSG).
    * `versions`, an array of versions that either contain for Login:
        * `type`, either `specific` or `all`.
        * `version`, if `type` is specific. The specific version it is compatible with.
    * for World messages:
        * An object c
        * `type`, either `all`, `major`, `minor`, `patch`, or `exact`.
        * `version`, depending on `type`, it will contain:
            * if `all`, nothing
            * if `major`, only `major`
            * if `minor`, `major`, and `minor`
            * if `patch`, `major, `minor`, and `patch`
            * if `exact`, `major, `minor`, `patch`, and exact


