extern crate completer;

use completer::Completer;

fn main() {
    let mut completer = Completer::new();
    completer.insert("abc");
    completer.insert("adc");
    completer.insert("abca");
    completer.insert("abda");
    completer.insert("adta");

    println!("{:#?}", completer);

    let test = completer.complete("ab");
    println!("{:#?}", test);
}