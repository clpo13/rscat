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
use std::process;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // If there aren't any arguments, do nothing.
    // TODO: read from stdin instead
    if args.len() > 1 {
        // Check for help and version flags
        if args[1] == "--version" {
            println!("rscat 0.1.0 - Copyright (C) 2018 Cody Logan");
            println!("This is free software, and you are welcome to modify or redistribute it under");
            println!("the terms of the GNU GPL Version 3 or later <http://gnu.org/licenses/gpl.html>.");
            println!("This program comes with ABSOLUTELY NO WARRANTY to the extent permitted by law.");
            process::exit(0);
        }
        if args[1] == "--help" {
            println!("Usage: rscat file1 file2 ...");
            println!("Prints the contents of one or more files to stdout.");
            println!();
            println!("  --help     prints this message and quits");
            println!("  --version  prints version info and quits");
            process::exit(0);
        }

        // Read one or more files
        // TODO: more user-friendly error handling
        for i in &args[1..] {
            let filename = i;

            let mut f = File::open(filename).expect("file not found");

            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("unable to read file");
            
            print!("{}", contents);
        }
    }
}
