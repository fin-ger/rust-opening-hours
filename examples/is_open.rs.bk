// is_open - A simple cli to check whether a is opened at a given time.
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
extern crate chrono;

fn usage() {
    // TODO
    println!("Please provide exactly 6 arguments!
Usage: [span] [point] [span_fmt] [start_fmt] [end_fmt] [point_fmt]");
}

fn main() {
    let mut args = std::env::args();

    if args.len() != 7 {
        usage();
        std::process::exit(1);
    }

    args.next();
    std::process::exit(0);
}
