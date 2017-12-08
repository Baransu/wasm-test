#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize)]
struct Person {
    name: String,
}

js_serializable!(Person);

fn say_hello(person: Person) {
    println!("Hello from Rust {}!", person.name);
}

fn main() {
    stdweb::initialize();

    let name = String::from("Tomek!");
    say_hello(Person { name: name });

    stdweb::event_loop();
}
