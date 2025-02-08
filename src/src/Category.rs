use std::collections::HashMap;

pub fn get(
    file: &HashMap<String, Vec<String>>,
    category: &str,
) -> Result<Vec<String>, std::io::Error> {
    match file.get(category) {
        Some(value) => Ok(value.clone()),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Category '{}' not found", category),
        )),
    }
}

pub fn add_to(file: &mut HashMap<String, Vec<String>>, category: &str, line: &str) {
    file.entry(category.to_string())
        .or_insert_with(Vec::new)
        .push(line.to_string());
}

pub fn delete_from(file: &mut HashMap<String, Vec<String>>, category: &str, line: &str) {
    if let Some(entries) = file.get_mut(category) {
        entries.retain(|entry| entry != &line);
        if entries.is_empty() {
            file.remove(category);
        }
    }
}
