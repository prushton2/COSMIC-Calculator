use cosmic::widget::{button, container};
use cosmic::Element;

use crate::ui::{Message, Page};

pub fn render() -> Element<'static, Message> {
    return container(button("Test")).into();
}
