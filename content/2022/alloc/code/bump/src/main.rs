use bumpalo::Bump;

enum Fur {
    White,
    Black,
    Colorful,
}

struct Kitty {
    name: String,
    age: u8,
    fur: Fur,
}

fn main() {
    // Create a new arena to bump allocate into.
    let bump = Bump::new();

    // Allocate values into the arena.
    let oskar = bump.alloc(Kitty {
        name: "Oskar".to_string(),
        age: 1,
        fur: Fur::White,
    });

    let flecki = bump.alloc(Kitty {
        name: "Flecki".to_string(),
        age: 10,
        fur: Fur::Colorful,
    });


    // Use the allocated values.
    println!("{} is {} years old", oskar.name, oskar.age);
    println!("{} is {} years old", flecki.name, flecki.age);

    // The arena is dropped at the end of the scope (e.g. function), freeing all
    // the allocated values.
}
