# GmSurveyQuestion

## Client Version 1.12

### Wowm Representation
```rust,ignore
struct GmSurveyQuestion {
    u32 question_id;
    u8 answer;
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 4 / Little | u32 | question_id |  | cmangos: questions found in GMSurveyQuestions.dbc |
| 0x04 | 1 / - | u8 | answer |  | Rating: hardcoded limit of 0-5 in pre-Wrath, ranges defined in GMSurveyAnswers.dbc Wrath+ |

