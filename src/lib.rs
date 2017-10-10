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

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(test)]
extern crate serde_json;

#[cfg(test)]
extern crate chrono_tz;

extern crate chrono;
extern crate timespan;

#[cfg(test)]
mod opening_hours_test;

mod opening_hours;

pub use self::opening_hours::OpeningHours;

// TODO:
// - add serde
// - add serde tests
// - add integration test
// - finish example
// - write documentation
// - tag and publish on crates.io
