use chrono::{DateTime, Duration, FixedOffset, Offset, Utc};
use chrono_tz::Europe::Paris;

pub fn now_paris_fixed(offset: Duration) -> DateTime<FixedOffset> {
    let now_paris_tz = Utc::now().with_timezone(&Paris);
    let fixed = now_paris_tz.offset().fix(); // need for postgres compatibility
    (now_paris_tz + offset).with_timezone(&fixed)
}
