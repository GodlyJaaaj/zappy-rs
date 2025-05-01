use iced::Element;
use iced::widget::text;

#[derive(Debug, Clone)]
pub struct SettingsView {}

#[derive(Debug, Clone)]
pub enum SettingsMessage {}

impl Default for SettingsView {
    fn default() -> Self {
        Self {}
    }
}

impl SettingsView {
    pub fn update(&mut self, _message: SettingsMessage) {}

    pub fn view(&self) -> Element<SettingsMessage> {
        text("Settings WIP").into()
    }
}
