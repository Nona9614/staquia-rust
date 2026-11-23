use crate::{implementations::number_system::NumberSystem, traits::positions::Positions};

pub mod enums;
pub mod implementations;
pub mod structs;
pub mod traits;

fn main() {
    let system = NumberSystem::custom('!', 'a', 'z', 5, 15, 0.15);
    let nnnna = system.forge("nnnnaxzup");
    let nnnn0 = system.forge("nnnn!xzup");
    let nnnnz = system.forge("nnnnzx");
    // After
    let nnno0 = system.next(&nnnnz);
    println!("next: nnno! = {}", Positions::string_from(&system, &nnno0));
    // Before
    let nnnmz = system.previous(&nnnn0);
    println!(
        "previous: nnnmz = {}",
        Positions::string_from(&system, &nnnmz)
    );
    // Core
    let nnnnn = system.core(&nnnna, &nnnnz);
    println!("core: nnnnn = {}", Positions::string_from(&system, &nnnnn));
}
