//! Creates positions with a system: `cargo run --example basic`.

use staquia::Staquia;

fn main() -> Result<(), staquia::Error> {
    let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;

    let start = system.start()?;
    let next = system.next(&start)?;
    let previous = system.previous(&start)?;
    let core = system.core(&start, &next)?;

    println!("previous: {previous}");
    println!("start:    {start}");
    println!("core:     {core}");
    println!("next:     {next}");

    // Positions stored as strings come back with `forge`
    let stored = system.forge(&core.to_string())?;
    assert_eq!(stored, core);
    Ok(())
}
