// opening-hours - Store opening hours of a service or place.
//
// Copyright (C) 2017
//     Fin Christensen <christensen.fin@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std;
use chrono::ParseError;
use chrono::naive::NaiveTime;

pub struct TimeSpan {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl std::str::FromStr for TimeSpan {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut span: Vec<&str> = s.splitn(2, " - ").collect();

        // make sure span[1], etc. do not go out of bounds.
        while span.len() < 2 {
            span.push("");
        }

        Ok(TimeSpan {
            start: NaiveTime::from_str(span[0])?,
            end: NaiveTime::from_str(span[1])?
        })
    }
}

impl std::fmt::Debug for TimeSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl std::fmt::Display for TimeSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "serde")]
mod serdes {
    use std::fmt;
    use super::TimeSpan;
    use serde::{ser, de};

    impl ser::Serialize for TimeSpan {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: ser::Serializer
        {
            struct FormatWrapped<'a, D: 'a> {
                inner: &'a D
            }

            impl<'a, D: fmt::Debug> fmt::Display for FormatWrapped<'a, D> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    self.inner.fmt(f)
                }
            }

            serializer.collect_str(&FormatWrapped { inner: &self })
        }
    }

    struct TimeSpanVisitor;

    impl<'de> de::Visitor<'de> for TimeSpanVisitor {
        type Value = TimeSpan;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
        {
            write!(formatter, "a formatted time span string")
        }

        fn visit_str<E>(self, value: &str) -> Result<TimeSpan, E>
            where E: de::Error
        {
            value.parse().map_err(|err| E::custom(format!("{}", err)))
        }
    }

    impl<'de> de::Deserialize<'de> for TimeSpan {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: de::Deserializer<'de>
        {
            deserializer.deserialize_str(TimeSpanVisitor)
        }
    }

    // TODO: add test for serialize
    // TODO: add test for deserialize
}
