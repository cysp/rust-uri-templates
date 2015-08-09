extern crate uritemplates;

fn main() {
    let r = uritemplates::parsing::parse("foo{bar}baz/{?quux}");
    println!("r: {:?}", r);
}
