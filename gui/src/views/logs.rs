use iced::Element;
use iced::widget::text;

#[derive(Debug)]
pub struct LogsView {}

#[derive(Debug, Clone)]
pub enum LogsMessage {}

impl Default for LogsView {
    fn default() -> Self {
        Self {}
    }
}

impl LogsView {
    pub fn update(&mut self, _message: LogsMessage) {}

    pub fn view(&self) -> Element<LogsMessage> {
        text("WIP Logs").into()
    }
}
