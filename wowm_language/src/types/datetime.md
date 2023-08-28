# DateTime

`u32` in the format of `years_after_2000 << 24 | month << 20 | month_day << 14 | weekday << 11 | hours << 6 | minutes`.

All values start on 0, and `weekday` starts on Sunday.
