use iced::widget::{button, text, column, row};
use iced::{Fill, Element};

//struct that sets the type of data that the counter object can take. structs are basically arrays/tuples that can be used as immutable datatypes
#[derive(Default)]
struct Counter{
    value: i64,
}

//enum type since the message can have multiple, but predetermined types
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn update (counter: &mut Counter, message: Message){
    match message{
        Message::Increment=>counter.value +=1,
        Message::Decrement=>counter.value -=1,
    }
}


fn view (counter: &Counter) -> Element<'_, Message> {
    column![
        button("Increment").on_press(Message::Increment),
        text(counter.value).size(20).center(),
        button("Decrement").on_press(Message::Decrement)


    ]  
    .padding(10)
    .into()
}



pub fn main() -> iced::Result{
    iced::run(update, view)
}