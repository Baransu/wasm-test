#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use stdweb::unstable::TryInto;
use stdweb::web::{IEventTarget, IElement, IHtmlElement, INode, HtmlElement, Element, document,
                  window};

use stdweb::web::event::{IEvent, IKeyboardEvent, DoubleClickEvent, ClickEvent, KeypressEvent,
                         ChangeEvent, BlurEvent, HashChangeEvent};

use stdweb::web::html_element::InputElement;

// #[derive(Serialize)]
// struct Person {
//     name: String,
// }

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

// js_serializable!(Person);

// fn say_hello(person: Person) {
//     println!("Hello from Rust {}!", person.name);
// }

fn main() {
    stdweb::initialize();

    let mut counter = 0;

    let root = document().get_element_by_id("root").unwrap();

    // let name = String::from("Tomek!");
    // say_hello(Person { name: name });

    let button = document().create_element("button");
    let count_display = document().create_element("span");

    button.append_child(&document().create_text_node("click me"));
    button.add_event_listener(enclose!((count_display) move |_: ClickEvent| {
        counter += 1;
        println!("Button clicked {} times!", counter);
        count_display.set_text_content(counter.to_string().as_str());
    }));

    count_display.set_text_content(counter.to_string().as_str());

    root.append_child(&button);
    root.append_child(&count_display);

    stdweb::event_loop();
}
