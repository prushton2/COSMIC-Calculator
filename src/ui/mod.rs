// dont really know what to call this but it contains stuff to prevent circular imports

use crate::config::Config;

#[derive(Debug, Clone)]
pub enum Message {
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    SetPage(Page)
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
