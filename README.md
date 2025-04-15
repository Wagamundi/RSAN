[![Crates.io](https://shields.io/crates/v/rsan)](https://crates.io/crates/rsan)

# RSAN

A File Format for easy, fast and lightweight Array notation.

## General Information

- Grew on a Notepad Projekt
- Is there to Help save arrays in a easy to read way
- makes it possible to edit data fast in a normal text editor
- Completly written in Rust
- Isn´t there to replace things like JSON or YML

## Examples:

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
pub async fn example() -> () {
        let file = match rsan::RSANFile::read("Test.rsan") {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(f) => f,
    };
}
```

Usage (Writing)

```rust
pub async fn example() -> () {
    let mut file = rsan::RSANFile::new();

    file.add_to("Test", "First");
    file.add_to("Test", "Second");

    file.write("Test.rsan").unwrap();
}

```

Usage (deleting and get)

```rust
pub async fn example() -> () {
    let file = match rsan::RSANFile::read("Test.rsan") {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(f) => f,
    };

    let get = file.get("Test");

    println!("{:?}", get); // ["First", "Second"]
}
```

## Flaws

- Possibly some, but for my usecase, I couldn't find any, so please report them to me
-Sometimes there are problems with '"' at the start and end of an vec element.

## License

RSAN is dual-licensed under Apache-2.0 and MIT.
