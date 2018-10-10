// rscat - prints the contents of given files to stdout
// Copyright (C) 2018 Cody Logan
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
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // If there aren't any arguments, do nothing.
    // TODO: read from stdin instead
    if args.len() > 1 {
        for i in &args[1..] {
            let filename = i;

            let mut f = File::open(filename).expect("file not found");

            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("unable to read file");
            
            println!("{}", contents);
        }
    }
}
