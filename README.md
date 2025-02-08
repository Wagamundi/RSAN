[![Crates.io](https://shields.io/crates/v/rsan)](https://crates.io/crates/rsan)

# RSAN
A File Format for easy, fast and lightweight Array notation.

# General Information
  - Grew on a Notepad Projekt
  - Is there to Help save arrays in a easy to read way
  - makes it possible to edit data fast in a normal text editor
  - Completly written in Rust
  - It possibly is ultra bad, but i will work on it from while to while
  - Isn´t there to replace things like JSON or YML

# Examples:
Structure
```rust
[Notes]
"Do laundry"
"Eat the dogs"
"Run away from Trump"

[Important Notes]
"Buy a new Coffe Machine"
"Cat food is needed ASAP"

```

Usage (Reading)
```rust
pub async fn read_notes() -> Vec<String> {
    let data_path = env::var("DATA_PATH").expect("DATA_PATH must be set");
    let file_name = env::var("FILE_NAME").expect("FILE_NAME must be set");

    let file_path = format!("{}/{}", data_path, file_name);

    let file = match rsan::File::read(&file_path) {
        Ok(file) => file,
        Err(_) => return vec![],
    };

    let mut notes = match rsan::Category::get(&file, "Notes") {
        Ok(notes) => notes,
        Err(_) => return vec![],
    };

    notes.reverse();

    notes
}
 ```

Usage (Writing)
```rust 
pub async fn post_note(req: web::Json<PostNoteRequest>) -> Result<impl Responder, error::Error> {
    let note = req.note.clone();
    let note = note.replace("\n", "<br>");

    let data_path = env::var("DATA_PATH").expect("DATA_PATH must be set");
    let file_name = env::var("FILE_NAME").expect("FILE_NAME must be set");

    let file_path = format!("{}/{}", data_path, file_name);

    let mut file = match rsan::File::read(&file_path) {
        Ok(file) => file,
        Err(_) => rsan::File::new(),
    };

    rsan::Category::add_to(&mut file, "Notes", &note);

    if !fs::metadata(&data_path).is_ok() {
        fs::create_dir_all(&data_path).map_err(|e| {
            error::ErrorInternalServerError(format!("Failed to create directory: {}", e))
        })?;
    }

    rsan::File::write(&file, &file_path)
        .map_err(|e| error::ErrorInternalServerError(format!("Failed to write file: {}", e)))?;

    Ok(HttpResponse::Ok().finish())
}

```

Usage (deleting and get)

```rust
pub async fn example() -> () {
    let data_path = env::var("DATA_PATH").expect("DATA_PATH must be set");
    let file_name = env::var("FILE_NAME").expect("FILE_NAME must be set");

    let file_path = format!("{}/{}", data_path, file_name);

    let mut file = match rsan::File::read(&file_path) {
        Ok(file) => file,
        Err(_) => rsan::File::new(),
    };

    let notes = match rsan::Category::get(&file, "Notes") {
        Ok(notes) => notes,
        Err(_) => vec![],
    };

    rsan::Category::delete_from(&mut file, "Notes", "I need to find Hobbys");
}
```

# Flaws
  - Deletes ' " ' at the end of an Value, just put a space there then it wont delete them.
  - If you find some tell me

# TO-DO list
  - Adding Comment functunality
  - Adding function to write a whole Array at once and don´t have to loop through it
  - Fixing Bugs

## License

RSAN is dual-licensed under Apache-2.0 and MIT.
