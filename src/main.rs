mod tokenizer;
mod shunter;
mod solver;

use iced::widget::{Button, text, Text, Grid, Column, Container, Row, column, container, row};
use iced::{Alignment, Fill, Element, Theme, Renderer, Length, FillPortion, Settings, Size, window};

const BUTTON_SIZE: u32 = 50;

//struct that sets the type of data that the object can take. structs are basically arrays/tuples that can be used as immutable datatypes
#[derive(Default)]
struct TextStack{
    value: String,
}




#[derive(Default)]
struct Calculator{
    value: String,
}

//enum type since the message can have multiple, but predetermined types
#[derive(Debug, Clone)]
enum Message {
    Input(String),
    Evaluate,
    Clear,
}


impl Calculator{

    fn new() -> Self{
        Self{
            value: "".to_string(),
        }
    }

    fn update (&mut self, message: Message){
        match message{
            Message::Input(ValueIn)=>{
                self.value.push_str(&ValueIn);
            },
            Message::Evaluate=>{
                let out = self.evaluate();
                self.value= (out.to_string());
            },
            Message::Clear=>{
                self.value="".to_string();
            },
        }
        
    }

    fn view (&self) -> Element<'_, Message> {

        let text_box = text::<Theme, Renderer>(self.value.clone()).size(40).height(Length::Fill).width(Length::Fill);

        let mut grid = Column::new().spacing(10).height(Length::FillPortion(2)).width(Length::Fill);

        for i in 0..4{
            let mut row = Row::new().spacing(10);

            for j in 0..4{
                row = row.push(
                    match (i,j){
                        (0,0) => {
                            Button::new(
                                Container::new(
                                    Text::new("7")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("7".to_string()))
                        },
                        (0,1) => {
                            Button::new(
                                Container::new(
                                    Text::new("8")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("8".to_string()))
                        },
                        (0,2) => {
                            Button::new(
                                Container::new(
                                    Text::new("9")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("9".to_string()))
                        },
                        (0,3) => {
                            Button::new(
                                Container::new(
                                    Text::new("/")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("/".to_string()))
                        },
                        (1,0) => {
                            Button::new(
                                Container::new(
                                    Text::new("4")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("4".to_string()))
                        },
                        (1,1) => {
                            Button::new(
                                Container::new(
                                    Text::new("5")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("5".to_string()))
                        },
                        (1,2) => {
                            Button::new(
                                Container::new(
                                    Text::new("6")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("6".to_string()))
                        },
                        (1,3) => {
                            Button::new(
                                Container::new(
                                    Text::new("*")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("*".to_string()))
                        },
                        (2,0) => {
                            Button::new(
                                Container::new(
                                    Text::new("1")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("1".to_string()))
                        },
                        (2,1) => {
                            Button::new(
                                Container::new(
                                    Text::new("2")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("2".to_string()))
                        },
                        (2,2) => {
                            Button::new(
                                Container::new(
                                    Text::new("3")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("3".to_string()))
                        },
                        (2,3) => {
                            Button::new(
                                Container::new(
                                    Text::new("-")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("-".to_string()))
                        },
                        (3,0) => {
                            Button::new(
                                Container::new(
                                    Text::new("0")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("0".to_string()))
                        },
                        (3,1) => {
                            Button::new(
                                Container::new(
                                    Text::new("AC")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Clear)
                        },
                        (3,2) => {
                            Button::new(
                                Container::new(
                                    Text::new("=")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Evaluate)
                        },
                        (3,3) => {
                            Button::new(
                                Container::new(
                                    Text::new("+")
                                )
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(Message::Input("+".to_string()))
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
            //.width(Length::Fill)
            
        //.width(Length::Fill)
        .padding(10)
        .into()

    }

    fn evaluate(&self) -> i64{

        let input = self.value.clone();
        let tokens = tokenizer::tokenize(&input);
        let shunted = shunter::shunt(tokens);
        return solver::evaluate(shunted);

    }
}


pub fn main() -> iced::Result{
   
    iced::application(Calculator::new, Calculator::update, Calculator::view)
        .window_size(iced::Size::new(400.0, 500.0))
        .run()
}