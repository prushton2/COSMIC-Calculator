// dont really know what to call this but it contains stuff to prevent circular imports

use cosmic::widget::{button, Button, container, text, Text};
use cosmic::iced::widget::{column, row};
use cosmic::iced;
use cosmic::Element;

use cosmic::iced_winit::graphics::core::{Pixels, Length, Padding};
//use cosmic::iced_winit::graphics::core::Vector;
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
    n0
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
   text(label)
       .into()
   // button(label)
   //     .on_press(Message::KeyPressEvent(key))
   //     .into()
}

pub fn base_calc() -> Element<'static, Message> {
    let spacing: f32 = 12.0;
    return container(
        column![
            row![
                styled_button("7", Key::n7),
                styled_button("8", Key::n8),
                styled_button("9", Key::n9),
            ].spacing(Pixels(spacing)),
            row![
                styled_button("4", Key::n4),
                styled_button("5", Key::n5),
                styled_button("6", Key::n6),
            ].spacing(Pixels(spacing)),
            row![
                styled_button("1", Key::n1),
                styled_button("2", Key::n2),
                styled_button("3", Key::n3),
            ].spacing(Pixels(spacing)),
        ].spacing(Pixels(spacing))
        
    ).into();
}
