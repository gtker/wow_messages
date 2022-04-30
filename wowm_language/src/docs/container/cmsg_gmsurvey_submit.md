## Client Version 1.12

```rust,ignore
cmsg CMSG_GMSURVEY_SUBMIT = 0x32A {
    u32 survey_id;    
    GmSurveyQuestion[10] questions;    
    CString answer_comment;    
}

```
