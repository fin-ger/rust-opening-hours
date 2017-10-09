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
use timespan::DateSpan;
use chrono_tz::Europe::Berlin;

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
            DateSpan::from_utc_datespan(&"2017-06-01 - 2017-09-01".parse().unwrap(), &Berlin),
            DateSpan::from_utc_datespan(&"2017-10-01 - 2018-01-01".parse().unwrap(), &Berlin),
        ],
    );
}

#[test]
fn contains_datetime_test() {
    // TODO
}

#[test]
fn contains_span_test() {
    // TODO
}

#[cfg(feature = "with-serde")]
mod with_serde {
    #[test]
    fn serialize_test() {
        // TODO
    }

    #[test]
    fn deserialize_test() {
        // TODO
    }
}
