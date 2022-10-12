# IR Specification

The Intermediate Representation is a JSON file that contains the members:

* `version`, an object that contains the SemVer of the file.
* `flags`, an array of [flags](definers.md).
* `enums`, an array of [enums](definers.md).
* `containers`, an array of [containers](containers.md).

The header JSON file without any objects in it would look like:
```json
{
  "version": {
    "major": 0,
    "minor": 0,
    "patch": 0
  },
  "flags": [],
  "enums": [],
  "containers": []
}
```
