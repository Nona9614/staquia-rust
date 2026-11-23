//! Keeps a list and its positions in sync: `cargo run --example list`.

use staquia::{Item, LexicalList, Position, Staquia};

struct Task {
    id: u32,
    title: &'static str,
    position: String,
}

impl Item for Task {
    type Id = u32;

    fn id(&self) -> &u32 {
        &self.id
    }

    fn position(&self) -> &str {
        &self.position
    }

    fn set_position(&mut self, position: &Position) {
        self.position = position.to_string();
    }
}

fn main() -> Result<(), staquia::Error<u32>> {
    let mut list = LexicalList::new(Staquia::default());
    for (id, title) in [(1, "Write"), (2, "Review"), (3, "Publish")] {
        list.push(Task { id, title, position: String::new() })?;
    }

    // Moves "Publish" between "Write" and "Review": only its row needs to be saved
    let moved = list.move_to(&3, Some(&1), Some(&2))?;
    println!("save {} -> {:?}", moved.value().title, moved.value().position);

    for item in &list {
        println!("{:>8}  {}", item.value().title, item.value().position);
    }
    Ok(())
}
