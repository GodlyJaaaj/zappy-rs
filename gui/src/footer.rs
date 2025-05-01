use iced::alignment::Vertical;
use iced::widget::{container, row, text};
use iced::{Element, Length, Padding, Pixels};

pub struct Footer {}

#[derive(Clone, Debug)]
pub struct FooterMessage;

impl Default for Footer {
    fn default() -> Self {
        Self { /* fields */ }
    }
}

impl Footer {
    pub fn update(&mut self, _message: FooterMessage) {}

    pub fn view(&self) -> Element<FooterMessage> {
        container(
            row![text("Footer")]
                .spacing(Pixels::from(10))
                .padding(Padding::from([0, 10]))
                .align_y(Vertical::Center),
        )
        .center(Length::Fill)
        .height(40)
        .style(container::rounded_box)
        .into()
    }
}
