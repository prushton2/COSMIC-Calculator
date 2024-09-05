use cosmic::widget::{button, container};
use cosmic::iced::widget::{column, row};
use cosmic::Element;

use cosmic::iced_winit::graphics::core::{Size, Pixels, Length, Padding};
use cosmic::iced_winit::graphics::core;


use crate::ui::{Message, Page, styled_button, styled_input, Key};

pub fn render(this: &AppModel) -> Element<'static, Message> {
    let spacing: f32 = 12.0;
    return container(
        column![
            row![
                styled_input(&this),
            ],
            row![
                styled_button("7", Key::n7),
                styled_button("8", Key::n8),
                styled_button("9", Key::n9),
                styled_button("+", Key::plus),
            ].spacing(Pixels(spacing)),
            row![
                styled_button("4", Key::n4),
                styled_button("5", Key::n5),
                styled_button("6", Key::n6),
                styled_button("-", Key::negative),
            ].spacing(Pixels(spacing)),
            row![
                styled_button("1", Key::n1),
                styled_button("2", Key::n2),
                styled_button("3", Key::n3),
                styled_button("*", Key::multiply),
            ].spacing(Pixels(spacing)),
            row![
                styled_button(".", Key::dot),
                styled_button("0", Key::n0),
                styled_button("=", Key::equals),
                styled_button("/", Key::divide),
            ].spacing(Pixels(spacing)),
        ].spacing(Pixels(spacing))
        
    ).into();

}
