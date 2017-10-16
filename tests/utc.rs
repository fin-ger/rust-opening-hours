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

extern crate opening_hours;
extern crate timespan;
extern crate chrono;

use chrono::{DateTime, Utc};
use opening_hours::OpeningHours;
use timespan::DateTimeSpan;

#[test]
fn utc() {
    let oh = OpeningHours::new(
        vec![
            "09:00:00 - 12:00:00".parse().unwrap(),
            "13:00:00 - 17:00:00".parse().unwrap(),
        ],
        vec!["Mon".parse().unwrap()],
        vec![
            "2017-06-01T00:00:00 +0200 - 2017-09-01T00:00:00 +0200"
                .parse::<DateTimeSpan<Utc>>()
                .unwrap(),
        ],
    );
    let contained = "2017-07-24T10:31:17 +0200"
        .parse::<DateTime<Utc>>()
        .unwrap();

    assert!(oh.contains_datetime(contained));
}
