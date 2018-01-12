extern crate speech_dispatcher;

use speech_dispatcher::*;

fn main() {
    let connection = speech_dispatcher::Connection::open("hello_world".to_string(), "hello_world".to_string(), "hello_world".to_string(), Mode::Single);
    connection.say(Priority::Important, format!("Hello, world at rate {}.", connection.get_voice_rate()));
    connection.set_voice_rate(100);
    connection.say(Priority::Important, "This is faster.".to_string());
    connection.set_voice_rate(0);
    connection.set_spelling(true);
    connection.say(Priority::Important, "This is spelled.".to_string());
    connection.set_spelling(false);
    connection.set_punctuation(Punctuation::All);
    connection.say(Priority::Important, "This statement, unlike others, has punctuation that is spoken!".to_string());
    connection.set_punctuation(Punctuation::None);
}
