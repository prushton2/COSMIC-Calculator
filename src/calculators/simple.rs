use cosmic::widget::{button, container};
use cosmic::Element;

use crate::ui::{Message, Page, base_calc};

pub fn render() -> Element<'static, Message> {
    return base_calc();
}
