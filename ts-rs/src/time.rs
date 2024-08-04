// still want to implement for `Instant`
#![allow(deprecated)]

use time::{
    Date, Duration, Instant, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday,
};

use crate::{impl_primitives, TS};

impl_primitives!(PrimitiveDateTime, Month, Weekday, Duration, Instant, OffsetDateTime, Time, UtcOffset, Date => "string");
