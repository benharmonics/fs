use clap::{arg, Arg, Command, ArgMatches};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::{fs, io, cmp, error};
use std::io::Write;
use std::path::PathBuf;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;

/* CLI argument parsing via clap crate */
pub fn args() -> ArgMatches {
    Command::new("readdir")
        .version("1.0")
        .author("benharmonics")
        .about("Reads items in a given directory")
        .arg(arg!(-a --all "Show hidden files"))
        .arg(arg!(-U --unsorted "Don't sort items (use directory ordering)"))
        .arg(arg!(-r --reverse "Reverse output order"))
        .arg(arg!(-s --size "Get file size (in bytes)"))
        .arg(Arg::new("human-readable")
            .short('h')
            .long("human-readable")
            .help("With '-s', gives size in a human-readable format (kB, GB, etc)"))
        .arg(arg!([DIRECTORY] ... "One or more directories to read"))
        .get_matches()
}

/* Takes a file size in bytes and formats a string using the most 'readable' unit */
fn pretty_filesize(num: u64) -> String {
    let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let delimiter = 1024_f64;
    let exponent = cmp::min(((num as f64).ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
    let pretty_bytes = format!("{:.2}", (num as f64) / delimiter.powi(exponent)).parse::<f64>().unwrap() * 1_f64;
    let unit = units[exponent as usize];
    format!("{} {}", pretty_bytes, unit)
}

/* Reads the directory contents and prints them to stdout */
fn write_to_stdout(stdout: &mut StandardStream, buf: PathBuf, flags: &HashMap<char, bool>)
                   -> Result<(), Box<dyn error::Error>> {
    let all_entries: Vec<PathBuf> = fs::read_dir(buf.as_path())
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, io::Error>>()
        .unwrap_or(vec![]);

    // We're going to print directories before files, so we separate them into two vectors now.
    let mut dirs = Vec::new();
    let mut files = Vec::new();

    // Separate all_entries into directories and files, to be printed separately
    for entry in all_entries {
        // Ignore hidden files
        if !flags[&'a'] && entry.file_name().unwrap().to_str().unwrap().starts_with('.') { continue; }
        if entry.is_dir() {
            dirs.push(entry);
        } else {
            files.push(entry);
        }
    }

    // Sorting alphabetically
    if !flags[&'U'] {
        dirs.sort(); files.sort();
        // Reverse, if reverse flag is set
        if flags[&'r'] { dirs.reverse(); files.reverse(); }
    }


    // Get just the filename/dirname from each PathBuf and collect them into vectors
    let filenames: Vec<&str> = files.iter()
        .map(|p| p.file_name().unwrap())
        .map(|s| s.to_str().unwrap())
        .collect();
    let dirnames: Vec<&str> = dirs.iter()
        .map(|p| p.file_name().unwrap())
        .map(|s| s.to_str().unwrap())
        .collect();

    for i in 0..dirs.len() {
        // Setting the correct color
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))?;

        // Getting file metadata
        let attrs = dirs[i].metadata()?;
        // Write file size to console if 's' flag used
        if flags[&'s'] {
            let size = if !flags[&'h'] { format!("{} B", attrs.len()) } else { pretty_filesize(attrs.len()) };
            write!(&mut *stdout, "({}) ", size)?;
        }
        writeln!(&mut *stdout, "{}", dirnames[i])?;
    }
    for i in 0..files.len() {
        // Check for broken symbolic links or inaccessible metadata before continuing
        if !files[i].exists() {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut *stdout, "{}", filenames[i])?;
            continue;
        }
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        let attrs = files[i].metadata().unwrap();
        if flags[&'s'] {
            let size = if !flags[&'h'] { format!("{} B", attrs.len()) } else { pretty_filesize(attrs.len()) };
            write!(&mut *stdout, "({}) ", size)?;
        }
        // Print filename in green if it's executable
        if attrs.permissions().mode() & 0o111 != 0 { stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?; }
        writeln!(&mut *stdout, "{}", filenames[i])?;
    }

    Ok(())
}

/* Function is called from main.rs; program exits with an error if anything fails. */
pub fn run(args: clap::ArgMatches) -> Result<(), Box<dyn error::Error>> {
    // flags parsed from arguments, normal CLI stuff
    let flags = HashMap::from([
        ('a', args.is_present("all")),
        ('r', args.is_present("reverse")),
        ('U', args.is_present("unsorted")),
        ('s', args.is_present("size")),
        ('h', args.is_present("human-readable")),
    ]);

    // Set up stdout stream (as opposed to a buffer)
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // If user entered no optional paths to be read, just read the current directory.
    let dirs: Option<_> = args.values_of("DIRECTORY");
    if dirs.is_none() {
        let current_dir = std::env::current_dir()?;
        write_to_stdout(&mut stdout, current_dir, &flags)?;
    } else {
        for dir in dirs.unwrap().collect::<Vec<_>>() {
            let dir_path = fs::canonicalize(dir)?;
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;     // font color changes
            writeln!(&mut stdout, " ==> {} <== ", dir_path.as_os_str().to_str().unwrap())?;
            write_to_stdout(&mut stdout, dir_path, &flags)?;
        }
    }

    Ok(())
}
