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
use std::io;
use std::io::prelude::*;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // If there aren't any arguments, do nothing.
    // TODO: read from stdin instead
    if args.len() > 1 {
        // Check for help and version flags
        if args[1] == "--version" {
            println!("rscat {} - Copyright (C) 2018 Cody Logan", VERSION);
            println!("This is free software, and you are welcome to modify or redistribute it under");
            println!("the terms of the GNU GPL Version 3 or later <http://gnu.org/licenses/gpl.html>.");
            println!("This program comes with ABSOLUTELY NO WARRANTY to the extent permitted by law.");
            process::exit(0);
        }
        if args[1] == "--help" {
            println!("Usage: rscat file1 file2 ...");
            println!("Prints the contents of one or more files to stdout.");
            println!();
            println!("  --help     print this help message");
            println!("  --version  output the program version number and license information");
            process::exit(0);
        }

        // Read one or more files
        for i in &args[1..] {
            let filename = i;

            // Print the file contents or an error message
            match read_from_file(filename) {
                Ok(contents) => {
                    print!("{}", contents);
                },
                Err(error) => {
                    eprintln!("{}: {}", filename, error);
                    continue;
                }
            };
        }
    }
}

// Try to open and read from a given file
// Returns the contents of the file or an error
fn read_from_file(filename: &String) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_and_compare() {
        use read_from_file;
        let test_string = "hello,\n  world!";
        let testfile = "tests/test.txt".to_string();
        assert_eq!(read_from_file(&testfile).unwrap(), test_string);
    }
}
