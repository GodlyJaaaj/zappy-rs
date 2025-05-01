use iced::alignment::Vertical;
use iced::widget::{container, row, text};
use iced::{Element, Length, Padding, Pixels};
use std::net::SocketAddrV4;

pub struct Footer {
    connection_status: Option<ConnectionStatus>,
}

#[derive(Clone, Debug)]
pub enum FooterMessage {
    ConnectionStatusChanged(ConnectionStatus),
}

#[derive(Clone, Debug)]
pub enum ConnectionStatus {
    Connected(SocketAddrV4),
    Disconnected,
    ConnectionFailed(String),
}

impl Default for Footer {
    fn default() -> Self {
        Self {
            connection_status: None,
        }
    }
}

impl Footer {
    pub fn update(&mut self, message: FooterMessage) {
        match message {
            FooterMessage::ConnectionStatusChanged(status) => {
                self.connection_status = Some(status);
            }
        }
    }

    pub fn view(&self) -> Element<FooterMessage> {
        let status_text = match &self.connection_status {
            Some(ConnectionStatus::Connected(addr)) => text(format!("Connected to {}", addr))
                .style(text::success)
                .width(Length::Fill),
            Some(ConnectionStatus::Disconnected) => text("Disconnected")
                .style(text::secondary)
                .width(Length::Fill),
            Some(ConnectionStatus::ConnectionFailed(error)) => {
                text(format!("Connection failed: {}", error))
                    .style(text::danger)
                    .width(Length::Fill)
            }
            None => text("Waiting...").width(Length::Fill),
        };

        container(
            row![status_text]
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
