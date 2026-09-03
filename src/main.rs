use iced::widget::{Button, text, Text, Grid, Column, Container, Row, column, container, row};
use iced::{Alignment, Fill, Element, Theme, Renderer};

const BUTTON_SIZE: u32 = 10;

//struct that sets the type of data that the object can take. structs are basically arrays/tuples that can be used as immutable datatypes
#[derive(Default)]
struct TextStack{
    value: String,
}

//enum type since the message can have multiple, but predetermined types
#[derive(Debug, Clone)]
enum Message {
    Input(String),
    Evaluate,
}

fn update (stack: &mut TextStack, message: Message){
    match message{
        Message::Input(ValueIn)=>{
            stack.value.push_str(&ValueIn);
        },
        Message::Evaluate=>todo!(),
    }
}


fn view (stack: &TextStack) -> Element<'_, Message> {

    let text_box = text::<Theme, Renderer>(stack.value.clone()).size(20);

    let mut grid = Column::new().spacing(10);

    for i in 0..1{
        let mut row = Row::new().spacing(10);

        for j in 0..4{
            row = row.push(
                match (i,j){
                    (0,0) => {
                        Button::new(
                            Container::new(
                                Text::new("1")
                            )
                        )
                        .on_press(Message::Input("1".to_string()))
                    },
                    (0,1) => {
                        Button::new(
                            Container::new(
                                Text::new("+")
                            )
                        )
                        .on_press(Message::Input("+".to_string()))
                    },
                    (0,2) => {
                        Button::new(
                            Container::new(
                                Text::new("3")
                            )
                        )
                        .on_press(Message::Input("3".to_string()))
                    },
                    (0,3) => {
                        Button::new(
                            Container::new(
                                Text::new("=")
                            )
                        )
                        .on_press(Message::Evaluate)
                    },
                    (_,_) => {
                        Button::new(
                            Container::new(
                                Text::new("A")
                            )
                        )
                    }
                }
            )

        }
        grid = grid.push(row);
    }

    Column::new()
        .push(text_box)
        .push(grid)
    .into()







    // Grid::new()
    //     .columns(4)
    //     .push(
    //         Button::new(
    //             Text::new("1")
    //                 .align_x(Alignment::Center)
    //                 .align_y(Alignment::Center)
    //         )
    //         .width(BUTTON_SIZE)
    //         .height(BUTTON_SIZE)
    //     )
    //     .push(
    //         Button::new(
    //             Text::new("+")
    //                 .align_x(Alignment::Center)
    //                 .align_y(Alignment::Center)
    //         )
    //         .width(BUTTON_SIZE)
    //         .height(BUTTON_SIZE)
    //     )
    //     .push(
    //         Button::new(
    //             Text::new("2")
    //                 .align_x(Alignment::Center)
    //                 .align_y(Alignment::Center)
    //         )
    //         .width(BUTTON_SIZE)
    //         .height(BUTTON_SIZE)
    //     )
    //     .push(
    //         Button::new(
    //             Text::new("=")
    //                 .align_x(Alignment::Center)
    //                 .align_y(Alignment::Center)
    //         )
    //         .width(BUTTON_SIZE)
    //         .height(BUTTON_SIZE)
    //     )
        
    // .spacing(10)
    // .into()

    //     .spacing(10)
    // )
    // .padding(10)
    // .center_x(Fill)
    // .center_y(Fill)
    // .into()
}



pub fn main() -> iced::Result{
    iced::run(update, view)
}