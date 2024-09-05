// dont really know what to call this but it contains stuff to prevent circular imports

use cosmic::widget::{Id, button, Button, container, text, Text, Style, text_input};
use cosmic::iced::widget::{column, row};
use cosmic::iced;
use cosmic::iced_core;
use cosmic::Element;
use cosmic::widget::button::Builder;
use std::borrow::Cow;

use cosmic::theme;
use crate::ui::core::font::Weight;

use cosmic::iced_winit::graphics::core::{Size, Pixels, Length, Padding};
use cosmic::iced_winit::graphics::core;
//use cosmic::iced_winit::graphics::core::border::Radius;

use crate::config::Config;

#[derive(Debug, Clone)]
pub enum Message {
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    SetPage(Page),
    KeyPressEvent(Key)
}

#[derive(Debug, Clone, Copy)]
pub enum Page {
    Simple,
    Programmer,
    Scientific
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

#[derive(Debug, Clone)]
pub enum Key {
    n1,
    n2,
    n3,
    n4,
    n5,
    n6,
    n7,
    n8,
    n9,
    n0,

    dot,

    plus,
    negative,
    multiply,
    divide,

    equals
}

//pub const buttonAppearance: button::Appearance = button::Appearance {
//    shadow_offset: Vector {x: 0.0, y: 0.0},
//    background: None,
//    border_radius: Radius,
//    border_color: Color,
//};

//pub const buttonTheme: style::Button = style::Button::Custom {
//    active:     
//};

pub fn styled_button<'a>(label: &'a str, key: Key) -> Element<'a, Message> {
   button(label)
       .width(100)
       .height(100)
       .on_press(Message::KeyPressEvent(key))
       .into()
}


pub fn styled_input<'a>() -> Element<'a, Message> {
    text_input::TextInput::new(
        "placeholder",
        &this.textfield
    ).into()
}
