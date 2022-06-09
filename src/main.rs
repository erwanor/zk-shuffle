use rand_core::OsRng;
use safe_shuffle::SafeShuffler;

fn main() {
    println!("Implementation notebook");
    let scrambled = SafeShuffler::new(OsRng).shuffle((0..100).collect());
    scrambled.iter().for_each(|i| println!("{i}"));
}
