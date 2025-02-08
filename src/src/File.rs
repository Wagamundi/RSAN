use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufWriter, Write};

pub fn read(path: &str) -> Result<HashMap<String, Vec<String>>, std::io::Error> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_category: Option<String> = None;

    for line in reader.lines() {
        let line = line?.trim().to_string();

        if line.starts_with('[') && line.ends_with(']') {
            let category = line[1..line.len() - 1].to_string();
            current_category = Some(category.clone());
            data.entry(category).or_insert_with(Vec::new);
        } else if !line.is_empty() {
            if let Some(category) = &current_category {
                data.entry(category.clone())
                    .or_insert_with(Vec::new)
                    .push(line.trim_matches('"').to_string());
            }
        }
    }
    Ok(data)
}

pub fn write(data: &HashMap<String, Vec<String>>, path: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let mut writer = BufWriter::new(file);

    let category_index = data.iter().count();

    let mut index = 0;

    for (category, lines) in data.iter() {
        writeln!(writer, "[{}]", category)?;
        for line in lines {
            writeln!(writer, "\"{}\"", line)?;
        }
        index += 1;
        if index < category_index {
            writeln!(writer, "")?;
        }
    }
    Ok(())
}

pub fn new() -> HashMap<String, Vec<String>> {
    HashMap::new()
}
