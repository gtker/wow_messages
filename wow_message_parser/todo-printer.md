# Getting World shippable

* Correct codegen
  * Else Ifs:
    - if a ==, else { if b ==  } (`SMSG_SEND_MAIL_RESULT`)
    - if a == { if b == } (`SMSG_AUCTION_COMMAND_RESULT`)
* Working UpdateMask
  * Improve Docs

# Direct upgrades to Login

* Workable Intermediate Representation for Login

# Low Priority

* Do not allocate for statically sized messages
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
