# Getting World shippable

* Correct codegen
  * Else Ifs:
    - if a ==, else { if b ==  } (`SMSG_SEND_MAIL_RESULT`)
    - if a == { if b == } (`SMSG_AUCTION_COMMAND_RESULT`)
    - if a != { if a == } (`SMSG_INVENTORY_CHANGE_FAILURE`)
  * Flag else ifs:
    - {if else if else if} (`SMSG_UPDATE_OBJECT`)
    - {if else if else if}, {if}, {if}, {if} (`SMSG_UPDATE_OBJECT`)
* Working UpdateMask
  * Tests
* Test: Flags with ifs for `CMD_REALM_LIST_Server` version 8
* Test: Add support for Guids

# Direct upgrades to Login

* async-std support
* Tests for all login messages
* Workable Intermediate Representation for Login
* Single Read/Write call in read/write
  * Read/write const arrays?

# Low Priority

* MaximumPossibleSize for derived enums
* Make derived flags usable with members
* Workable Intermediate Representation for World
* cfg options for `wow_srp`
* More tests/definitions
* Add derives for all types
  * Add Eq and Hash for structs?
* MSRV test
  * Login
  * World
* Error on invalid flag enumerator used
* Error out on using == for flags and vice versa
* Wowm optional, elseif in doc comments (`DOCC` message output currently)
* Function to Write objects (eg strings) by reference
* Tokio support
