use iced::Element;
use iced::widget::text;

#[derive(Default)]
pub struct MapView {}

#[derive(Debug, Clone)]
pub enum MapMessage {}

impl MapView {
    pub fn update(&mut self, _message: MapMessage) {}

    pub fn view(&self) -> Element<MapMessage> {
        text("Map").into()
    }
}
