extern crate speech_dispatcher;

use speech_dispatcher::*;
use std::io;

fn main() {
    let connection = speech_dispatcher::Connection::open(
        "hello_world",
        "hello_world",
        "hello_world",
        Mode::Threaded,
    );
    connection.on_begin(Some(|id| println!("Beginning {}", id)));
    connection.on_end(Some(|id| println!("Ending {}", id)));
    connection.say(
        Priority::Important,
        format!("Hello, world at rate {}.", connection.get_voice_rate()),
    );
    connection.set_voice_rate(100);
    connection.say(Priority::Important, "This is faster.");
    connection.set_voice_rate(0);
    connection.set_spelling(true);
    connection.say(Priority::Important, "This is spelled.");
    connection.set_spelling(false);
    connection.set_punctuation(Punctuation::All);
    connection.say(
        Priority::Important,
        "This statement, unlike others, has punctuation that is spoken!",
    );
    connection.set_punctuation(Punctuation::None);
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}
