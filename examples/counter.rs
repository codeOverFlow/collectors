use collectors::Counter;
use std::iter::FromIterator;

fn main() {
    let some_text = String::from("a random text just to test Counter<char>");
    let mut counter: Counter<char> = Counter::new();
    counter.update_from_iter(some_text.chars());

    println!("Occurences for \"{}\" are {:#?}", some_text, counter);

    let some_int_vec: Vec<u8> = vec![
        1, 2, 3, 4, 5, 2, 5, 2, 1, 5, 6, 3, 7, 8, 9, 7, 5, 4, 9, 8, 9, 6, 6, 6, 3, 1, 5, 74, 7, 5,
        5, 2, 4, 5, 6, 2, 3, 6, 8, 5,
    ];
    let counter: Counter<&u8> = Counter::from_iter(some_int_vec.iter());
    println!("Occurences for {:?} are {:#?}", some_int_vec, counter);
    println!("Occurences for 5 are {}", counter[&5]);

    for (key, occurence) in counter.iter() {
        println!("Occurences for {:?} are {:?}", key, occurence);
    }
    println!("# of different elements: {}", counter.len());
}
