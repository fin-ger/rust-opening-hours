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

use OpeningHours;
use chrono::TimeZone;
use chrono_tz::Europe::Berlin;
use timespan::DateTimeSpan;

#[test]
fn new_test() {
    OpeningHours::new(
        vec![
            "09:00:00 - 12:00:00".parse().unwrap(),
            "13:00:00 - 17:00:00".parse().unwrap(),
        ],
        vec![
            "Mon".parse().unwrap(),
            "Tue".parse().unwrap(),
            "Wed".parse().unwrap(),
            "Fri".parse().unwrap(),
        ],
        vec![
            DateTimeSpan::from_utc_datetimespan(
                &"2017-06-01T00:00:00 - 2017-09-01T00:00:00".parse().unwrap(),
                &Berlin
            ),
            DateTimeSpan::from_utc_datetimespan(
                &"2017-10-01T00:00:00 - 2018-01-01T00:00:00".parse().unwrap(),
                &Berlin
            ),
        ],
    );
}

#[test]
fn contains_datetime_test() {
    let oh = OpeningHours::new(
        vec![
            "09:00:00 - 12:00:00".parse().unwrap(),
            "13:00:00 - 17:00:00".parse().unwrap(),
        ],
        vec![
            "Mon".parse().unwrap(),
            "Tue".parse().unwrap(),
            "Wed".parse().unwrap(),
            "Fri".parse().unwrap(),
        ],
        vec![
            DateTimeSpan::from_utc_datetimespan(
                &"2017-06-01T00:00:00 - 2017-09-01T00:00:00".parse().unwrap(),
                &Berlin
            ),
            DateTimeSpan::from_utc_datetimespan(
                &"2017-10-01T00:00:00 - 2018-01-01T00:00:00".parse().unwrap(),
                &Berlin
            ),
        ],
    );

    let contained = Berlin.from_utc_datetime(&"2017-07-12T11:00:00".parse().unwrap());
    let datetime_err = Berlin.from_utc_datetime(&"2017-07-12T12:30:00".parse().unwrap());
    let weekday_err = Berlin.from_utc_datetime(&"2017-07-13T11:00:00".parse().unwrap());
    let date_err = Berlin.from_utc_datetime(&"2017-04-12T11:00:00".parse().unwrap());

    assert!(oh.contains_datetime(contained));
    assert!(!oh.contains_datetime(datetime_err));
    assert!(!oh.contains_datetime(weekday_err));
    assert!(!oh.contains_datetime(date_err));
}

#[test]
fn contains_span_test() {
    let oh = OpeningHours::new(
        vec![
            "09:00:00 - 12:00:00".parse().unwrap(),
            "13:00:00 - 17:00:00".parse().unwrap(),
        ],
        vec![
            "Mon".parse().unwrap(),
            "Tue".parse().unwrap(),
            "Wed".parse().unwrap(),
            "Fri".parse().unwrap(),
        ],
        vec![
            DateTimeSpan::from_utc_datetimespan(
                &"2017-06-01T00:00:00 - 2017-09-01T00:00:00".parse().unwrap(),
                &Berlin
            ),
            DateTimeSpan::from_utc_datetimespan(
                &"2017-10-01T00:00:00 - 2018-01-01T00:00:00".parse().unwrap(),
                &Berlin
            ),
        ],
    );

    let contained = DateTimeSpan::from_utc_datetimespan(
        &"2017-07-12T10:00:00 - 2017-07-12T11:00:00".parse().unwrap(),
        &Berlin,
    );
    let partially_contained_datetime =
        DateTimeSpan::from_utc_datetimespan(
            &"2017-07-12T11:00:00 - 2017-07-12T12:30:00".parse().unwrap(),
            &Berlin,
        );
    let not_contained_datetime = DateTimeSpan::from_utc_datetimespan(
        &"2017-07-12T07:00:00 - 2017-07-12T08:00:00".parse().unwrap(),
        &Berlin,
    );
    let partially_contained_weekdays =
        DateTimeSpan::from_utc_datetimespan(
            &"2017-07-12T15:00:00 - 2017-07-13T10:00:00".parse().unwrap(),
            &Berlin,
        );
    let not_contained_weekdays = DateTimeSpan::from_utc_datetimespan(
        &"2017-07-13T10:00:00 - 2017-07-13T11:00:00".parse().unwrap(),
        &Berlin,
    );
    let partially_contained_datespan =
        DateTimeSpan::from_utc_datetimespan(
            &"2017-09-01T15:00:00 - 2017-09-02T10:00:00".parse().unwrap(),
            &Berlin,
        );
    let not_contained_datespan = DateTimeSpan::from_utc_datetimespan(
        &"2017-03-06T10:00:00 - 2017-04-06T11:00:00".parse().unwrap(),
        &Berlin,
    );

    assert!(oh.contains_span(&contained));
    assert!(!oh.contains_span(&partially_contained_datetime));
    assert!(!oh.contains_span(&not_contained_datetime));
    assert!(!oh.contains_span(&partially_contained_weekdays));
    assert!(!oh.contains_span(&not_contained_weekdays));
    assert!(!oh.contains_span(&partially_contained_datespan));
    assert!(!oh.contains_span(&not_contained_datespan));
}
