use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufWriter, Write};

#[derive(Debug)]
pub struct Category {
    pub entries: Vec<String>,
}

impl Category {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct RSANFile {
    categories: HashMap<String, Category>,
}

impl Default for RSANFile {
    fn default() -> Self {
        Self::new()
    }
}

impl RSANFile {
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
        }
    }

    pub fn read(path: &str) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut rsan_file = RSANFile::new();
        let mut current_category: Option<String> = None;

        for line in reader.lines() {
            let line = line?.trim().to_string();

            if line.starts_with('[') && line.ends_with(']') {
                let category_name = line[1..line.len() - 1].to_string();
                rsan_file
                    .categories
                    .insert(category_name.clone(), Category::new());
                current_category = Some(category_name);
            } else if line.starts_with("//") {
                continue;
            } else if !line.is_empty() {
                if let Some(category) = &current_category {
                    let clean_line = line
                        .split_once("//")
                        .map_or(line.clone(), |(text, _)| text.trim().to_string());
                    rsan_file.add_to(category, &clean_line.trim_matches('"'));
                }
            }
        }
        Ok(rsan_file)
    }

    pub fn write(&self, path: &str) -> Result<(), io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let mut writer = BufWriter::new(file);
        let category_count = self.categories.len();
        let mut index = 0;

        for (name, category) in &self.categories {
            writeln!(writer, "[{}]", name)?;
            for entry in &category.entries {
                writeln!(writer, "\"{}\"", entry)?;
            }
            index += 1;
            if index < category_count {
                writeln!(writer)?;
            }
        }
        Ok(())
    }

    pub fn add_to(&mut self, category: &str, entry: &str) {
        self.categories
            .entry(category.to_string())
            .or_insert_with(Category::new)
            .entries
            .push(entry.to_string());
    }

    pub fn delete_from(&mut self, category: &str, entry: &str) {
        if let Some(cat) = self.categories.get_mut(category) {
            cat.entries.retain(|e| e != entry);
            if cat.entries.is_empty() {
                self.categories.remove(category);
            }
        }
    }

    pub fn get(&self, category: &str) -> Vec<String> {
        self.categories
            .get(category)
            .map_or(Vec::new(), |c| c.entries.clone())
    }

    pub fn get_categorys(&self) -> Vec<String> {
        self.categories.keys().cloned().collect()
    }

    pub fn length(&self) -> usize {
        self.categories.len()
    }
}
