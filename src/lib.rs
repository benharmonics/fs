pub mod config;
mod output;

use std::{fs, env};
use termcolor::{BufferWriter, ColorChoice};

pub fn run(args: clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let flags = std::collections::HashMap::from([
        ('a', args.is_present("all")),
        ('r', args.is_present("reverse")),
        ('u', args.is_present("unsorted")),
        ('s', args.is_present("size")),
        ('h', args.is_present("human-readable")),
        ('b', args.is_present("base-1000")),
    ]);

    let bufwtr = BufferWriter::stdout(ColorChoice::Auto);
    let mut buffer = bufwtr.buffer();

    // Write to buffer
    if let Some(dirs) = args.values_of("DIRECTORY") {
        // First, the case where we have one or more "directory" arguments to be read
        for dir in dirs {
            let pathbuf = fs::canonicalize(dir)?;
            // Output dir name
            output::write_str_to_buffer(&mut buffer, pathbuf.to_str().unwrap_or("[Missing]"))?;
            // Output dir contents
            output::print_entries(&mut buffer, pathbuf.as_path(), &flags)?;
        }
    } else {
        // Now, the case where no arguments for "directory" have been given
        let pathbuf = env::current_dir()?;
        output::print_entries(&mut buffer, pathbuf.as_path(), &flags)?;
    }
    bufwtr.print(&buffer)?;

    Ok(())
}