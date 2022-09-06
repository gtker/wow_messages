* Correct codegen
  * Else Ifs:
    - if a ==, else { if b ==  } (`SMSG_SEND_MAIL_RESULT`)
    - if a == { if b == } (`SMSG_AUCTION_COMMAND_RESULT`)
* UpdateMask docs
  * Improve Docs

# Low Priority

* Max size of vanilla message (10240) mangoszero/server/WorldSocket.cpp:367

* Do not allocate for statically sized messages
* Special type for `SMSG_LOGIN_SETTIMESPEED.datetime`
* More tests/definitions
* Add derives for all types
  * Add Eq and Hash for structs?
* MSRV test
  * Login
  * World
* cfg options for `wow_srp`
