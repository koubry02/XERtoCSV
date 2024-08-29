use csv::Writer;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    // Get the directory paths from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_directory> <output_directory>", args[0]);
        std::process::exit(1);
    }

    let input_dir = &args[1];
    let output_dir = &args[2];

    // Ensure the input directory exists
    if !fs::metadata(input_dir)?.is_dir() {
        eprintln!("The provided input path is not a directory.");
        std::process::exit(1);
    }

    // Ensure the output directory exists or create it
    if !fs::metadata(output_dir).map(|m| m.is_dir()).unwrap_or(false) {
        fs::create_dir_all(output_dir)?;
    }

    // Iterate over all .xer files in the input directory
    for entry in WalkDir::new(input_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("xer") {
            process_file(path, output_dir)?;
            println!("{}", path.to_string_lossy())
        }
       
    }

    Ok(())
}

fn process_file(file_path: &Path, output_dir: &str) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    // Read the entire file content as bytes
    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;

    // Convert bytes to a string using a lossless conversion
    let content = String::from_utf8_lossy(&content);

    let base_name = file_path.file_stem().unwrap().to_str().unwrap();
    let output_subdir = Path::new(output_dir).join(base_name);

    // Ensure the output subdirectory exists
    if !fs::metadata(&output_subdir).map(|m| m.is_dir()).unwrap_or(false) {
        fs::create_dir_all(&output_subdir)?;
    }

    let mut section_name = String::new();
    let mut headers = Vec::new();
    let mut rows = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("%T") {
            if !section_name.is_empty() {
                save_csv(&section_name, &headers, &rows, &output_subdir)?;
            }

            section_name = line[2..].trim().to_string();
            headers.clear();
            rows.clear();
        } else if line.starts_with("%F") {
            headers = line[2..].trim().split('\t').map(String::from).collect();
        } else if line.starts_with("%R") {
            let row_values = line[2..].trim().split('\t').map(String::from).collect::<Vec<String>>();
            let mut row = row_values;
            if row.len() < headers.len() {
                // Fill missing values with empty strings
                row.extend(vec!["".to_string(); headers.len() - row.len()]);
            }
            rows.push(row);
        }
    }

    // Save the last section if it exists
    if !section_name.is_empty() {
        save_csv(&section_name, &headers, &rows, &output_subdir)?;
    }

    Ok(())
}

fn save_csv( section_name: &str, headers: &[String], rows: &[Vec<String>], output_subdir: &Path) -> std::io::Result<()> {
    // Create the CSV file path with the same base name and section name
    let csv_file_name = format!("{}.csv", section_name.replace(' ', "_"));
    let csv_file_path = output_subdir.join(csv_file_name);

    let file = File::create(csv_file_path)?;
    let mut wtr = Writer::from_writer(file);

    // Write headers and rows to the CSV file
    wtr.write_record(headers)?;

    for row in rows {
        wtr.write_record(row)?;
    }

    wtr.flush()?;

    Ok(())
}
