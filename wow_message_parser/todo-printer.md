# Todo

## Bugs

* Duplicate LoginResults because of versions

## Must

* Make derived flags usable with members
* JSON intermediate form
* Integer equals?
* UpdateMask
  - Tests
* Else Ifs:
  - if a ==, else { if b ==  }
  - if a == { if b == }
  - if a != { if a == }
* Flag else ifs:
  - {if else if else if}
  - {if else if else if}, {if}, {if}, {if}

## Nice to have

* cfg options for wow_srp
* New() function for all/write_as with references
* Add ref to types in declarations?
* More tests/definitions
* Flatten errors for structs
* Detect `self` enums and do not generate TryFrom error
* Add derives for all types
  * Add Eq and Hash for structs?
* MaximumPossibleSize for derived enums
* MSRV test
* Error on invalid flag enumerator used

## Optimizations
* Single Read/Write call in read/write
  * Read/write const arrays?
  
## Errors

* IfEdgeCases should error on missing SIX (Enumerator used in else if || doesn't exist)
* ElseIf for flags
* Using == for flags and vice versa

## TODO

* INT_EQUALS: Use arbitrary integer values in if statements

## Design

### Enums/Flags

* Name
* IntegerType
  * Name
  * Size
* Casted IntegerTypes
  * Name
  * Size
* Enumerators
    * Name
    * Value
      * Value
      * Original str
    * Tags
* [Optional] Self Value **Not for Flags**
  * Name
  * Tags
* Tags

### Structs/Messages

* Name
* Is Constant sized + Size
* Maximum possible size
* Object Type (Struct/Clogin/Slogin/Msg/Cmsg/Smsg)
* Statements
  * Member
    * Name
    * Type
      * Is constant sized + size
      * Maximum possible size
    * [Optional] Used as size in
      * Name
    * [Optional] Constant Value
      * Value
      * Original str
    * Tags
  * Statement
* Tags


* Type
  * Int (IntegerType)
  * Float (FloatType)
  * CString
  * String
    * Length
  * Array (Array)
  * Enumerator (all)
  * Flag (all)
  * Struct (all)
