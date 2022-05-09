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

# Direct upgrades to Login

* Workable Intermediate Representation for Login
* Single Read/Write call in read/write
  * Read/write const arrays?

# Low Priority

* Workable Intermediate Representation for World
* More tests/definitions
* Add derives for all types
  * Add Eq and Hash for structs?
* Error on invalid flag/enumerator used
* Error out on using == for flags and vice versa
* MSRV test
  * Login
  * World
* cfg options for `wow_srp`
* Function to Write objects (eg strings) by reference
