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
use timespan::{DateSpan, DateTimeSpan, NaiveTimeSpan};
use chrono::{TimeZone, Weekday, DateTime, Datelike};

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct OpeningHours<T: TimeZone> where <T as TimeZone>::Offset: std::marker::Copy + std::fmt::Display {
    pub open_hours: Vec<NaiveTimeSpan>,
    pub days_of_week: Vec<Weekday>,
    pub valid_days: Vec<DateSpan<T>>,
}

impl<T: TimeZone> OpeningHours<T>
where
    <T as TimeZone>::Offset: std::marker::Copy + std::fmt::Display {
    pub fn new(
        open_hours: Vec<NaiveTimeSpan>,
        days_of_week: Vec<Weekday>,
        valid_days: Vec<DateSpan<T>>
    ) -> OpeningHours<T> {
        OpeningHours {
            open_hours: open_hours,
            days_of_week: days_of_week,
            valid_days: valid_days,
        }
    }

    pub fn contains_datetime(&self, time: DateTime<T>) -> bool {
        let date = time.date();
        let weekday = date.weekday();

        if !self.days_of_week.iter().any(|d| *d == weekday) {
            return false;
        }

        if !self.valid_days.iter().any(|ds| ds.contains(&date)) {
            return false;
        }

        let local = time.naive_utc().time();

        self.open_hours.iter().any(|ts| ts.contains(&local))
    }

    pub fn contains_span(&self, span: &DateTimeSpan<T>) -> bool {
        let datespan = DateSpan {
            start: span.start.date(),
            end: span.end.date(),
        };
        let days = datespan.duration().num_days() + 1;
        let mut weekday = span.start.weekday();

        for _ in 0..days {
            if !self.days_of_week.iter().any(|d| *d == weekday) {
                return false;
            }

            weekday = weekday.succ();
        }

        if !self.valid_days.iter().any(|ts| ts.is_superset(&datespan)) {
            return false;
        }

        let localspan = NaiveTimeSpan {
            start: span.start.naive_utc().time(),
            end: span.end.naive_utc().time(),
        };

        self.open_hours.iter().any(|ts| ts.is_superset(&localspan))
    }
}

impl<T: TimeZone> std::fmt::Debug for OpeningHours<T>
where
    <T as TimeZone>::Offset: std::marker::Copy + std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hours = self.open_hours
            .iter()
            .map(|ref ts| format!("{}", ts))
            .collect::<Vec<String>>()
            .join(", ");
        let weekdays = self.days_of_week
            .iter()
            .map(|ref w| format!("{:#?}", w))
            .collect::<Vec<String>>()
            .join(", ");
        let valid_days = self.valid_days
            .iter()
            .map(|ref ds| format!("{:#?}", ds))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}\n{}\n{}", hours, weekdays, valid_days)
    }
}

impl<T: TimeZone> std::fmt::Display for OpeningHours<T>
where
    <T as TimeZone>::Offset: std::marker::Copy + std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
