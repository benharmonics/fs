use std::{fs, cmp, io, error};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use termcolor::{WriteColor, ColorSpec, Color};
use terminal_size::{Width, Height, terminal_size};

/// Prints all the items in a directory to stdout
pub fn print_entries<W: WriteColor>(
    buffer: &mut W, 
    path: &Path, 
    flags: &HashMap<char, bool>,
) -> Result<(), Box<dyn error::Error>> {
    let mut pathbufs = fs::read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, _>>()?;

    // Don't retain hidden files (that start with '.')
    if !flags[&'a'] {
        pathbufs.retain(|e| !e.file_name().unwrap().to_str().unwrap().starts_with('.'));
    }
    // Leave items unsorted if -U flag was used
    if !flags[&'u'] {
        pathbufs.sort();
    }
    // Reverse the items if -r flag was used
    if flags[&'r'] {
        pathbufs.reverse();
    }
    
    // A Vec of Paths from which we'll write to the buffer
    let entries: Vec<&Path> = pathbufs.iter().map(|b| b.as_path()).collect();

    // Writing to the buffer
    write_dir_contents_to_buffer(buffer, entries, flags)?;
    // A last newline for formatting
    writeln!(buffer, "").unwrap();

    Ok(())
}

fn write_dir_contents_to_buffer<W: WriteColor>(
    buffer: &mut W, 
    entries: Vec<&Path>, 
    flags: &HashMap<char, bool>,
) -> Result<(), Box<dyn error::Error>> {
    // I'm not sure if it's efficient to specify colors up front here...
    let mut blue = ColorSpec::new();
    let mut cyan = ColorSpec::new();
    let mut green = ColorSpec::new();
    let mut white = ColorSpec::new();
    let mut red = ColorSpec::new();
    blue.set_fg(Some(Color::Blue)).set_bold(true);
    cyan.set_fg(Some(Color::Cyan));
    green.set_fg(Some(Color::Green));
    white.set_fg(Some(Color::White));
    red.set_fg(Some(Color::Red)).set_bold(true);

    let length_of_longest_entry: usize = entries.iter()
        .map(|&e| e.file_name().unwrap().to_str().unwrap().len())
        .max()
        .unwrap();
    let buffer_width: usize = length_of_longest_entry + 2;
    let console_width: usize = console_width();
    let entries_per_line: usize = console_width / buffer_width;

    for (i, entry) in entries.iter().enumerate() {
        // File name
        let filename: &str = entry.file_name().unwrap_or(std::ffi::OsStr::new("")).to_str().unwrap_or("");
        // Handle missing files / broken symlinks
        if !entry.exists() {
            buffer.set_color(&red)?;
            if i % entries_per_line == entries_per_line - 1 
                && i != entries.len() - 1
            {
                writeln!(buffer, "{}", right_pad(filename, buffer_width))?;
            } else {
                write!(buffer, "{}", right_pad(filename, buffer_width))?;
            }
            continue;
        }

        // File metadata
        let attrs: fs::Metadata = entry.metadata()?;
        
        // Setting font colors
        buffer.set_color(&white)?;
        if entry.is_dir() {
            buffer.set_color(&blue)?;
        }
        if entry.is_symlink() {
            buffer.set_color(&cyan)?;
        } 
        if attrs.permissions().mode() & 0o111 != 0
            && !entry.is_dir()
        {
            buffer.set_color(&green)?;
        }

        // We will pad the file name to a fixed width
        let outstr = if buffer_width * entries.len() <= console_width {
            right_pad(filename, filename.len() + 2)
        } else {
            right_pad(filename, buffer_width)
        };

        // Printing out the file size throws off the whole formatting scheme, so it's a separate thing here.
        if flags[&'s'] {
            let file_size = if !flags[&'h'] { 
                format!("{} B", attrs.len())
            } else if flags[&'b'] { 
                human_readable_filesize(attrs.len(), true)
            } else {
                human_readable_filesize(attrs.len(), false)
            };
            write!(buffer, "{}", right_pad(&file_size, 10))?;
            writeln!(buffer, "{}", outstr)?;
            continue;
        }

        // Write to buffered line, or add a new line if we need the space.
        if i % entries_per_line == entries_per_line - 1 
            && i != entries.len() - 1 
        {
            writeln!(buffer, "{}", outstr)?;
        } else {
            write!(buffer, "{}", outstr)?;
        }
        buffer.set_color(&white)?;
    }

    Ok(())
}

pub fn write_str_to_buffer<W: WriteColor>(buffer: &mut W, s: &str) -> io::Result<()> {
    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    writeln!(buffer, "➥ {}", s)?;
    Ok(())
}

// Width of the terminal (in chars)
fn console_width() -> usize {
    let size = terminal_size();
    if let Some((Width(width), Height(_))) = size {
        return width as usize
    }
    eprintln!("Couldn't determine terminal width.");
    50
}

// Pad a string with spaces on the right side
fn right_pad(s: &str, width: usize) -> String {
    let mut res = String::from(s);
    while res.len() < width {
        res.push(' ');
    }
    res
}

// Prints file sizes like 4.14 kB, 2.1 GB, etc.
fn human_readable_filesize(num: u64, base_1000: bool) -> String {
    let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let delimiter = if base_1000 { 1000_f64 } else { 1024_f64 };
    let exponent = cmp::min(((num as f64).ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
    let pretty_bytes = format!("{:.2}", (num as f64) / delimiter.powi(exponent)).parse::<f64>().unwrap(); 
    let unit = units[exponent as usize];
    format!("{} {}", pretty_bytes, unit)
}
