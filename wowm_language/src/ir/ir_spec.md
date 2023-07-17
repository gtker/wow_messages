# IR Specification

The Intermediate Representation is a JSON file that contains the members:

* `version`, an object that contains the SemVer of the file.
* `login`, [flags](definers.md), [enums](definers.md), and [structs](containers.md) and [messages](containers.md) for login messages.
* `world`, [flags](definers.md), [enums](definers.md), and [structs](containers.md) and [messages](containers.md) for login messages.

The header JSON file without any objects in it would look like:
```json
{
  "version": {
    "major": 0,
    "minor": 0,
    "patch": 0
  },
  "login": {
    "flags": [],
    "enums": [],
    "containers": []
  },
  "world": {
    "flags": [],
    "enums": [],
    "containers": []
  }
}
```
