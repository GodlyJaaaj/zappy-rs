use crate::game::GameState;
use iced::alignment::Vertical;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::text::danger;
use iced::widget::{container, row, text, Scrollable};
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

    pub fn view<'a>(&self, game_state: &'a GameState) -> Element<'a, FooterMessage> {
        let status_text = match &self.connection_status {
            Some(ConnectionStatus::Connected(addr)) => {
                text(format!("Connected to {}", addr)).style(text::success)
            }
            Some(ConnectionStatus::Disconnected) => text("Disconnected").style(text::secondary),
            Some(ConnectionStatus::ConnectionFailed(error)) => {
                text(format!("Connection failed: {}", error)).style(text::danger)
            }
            None => text("Waiting..."),
        }
        .width(Length::Shrink);

        let team_display = if game_state.teams().is_empty() {
            row![text("No teams").style(danger)]
        } else {
            let team_texts = game_state
                .teams()
                .iter()
                .map(|(name, color)| {
                    let mut color_a_max = color.clone();
                    color_a_max.a = 1.0;
                    text(name).color(color_a_max).into()
                });

            row(team_texts).spacing(10)
        };

        container(
            row![
                status_text,
                container(Scrollable::with_direction(
                    team_display,
                    Direction::Horizontal(
                        Scrollbar::default()
                            .scroller_width(0)
                            .margin(0)
                            .spacing(0)
                            .width(0)
                    )
                ))
                .align_x(iced::alignment::Horizontal::Right)
                .width(Length::Fill)
            ]
            .spacing(Pixels::from(10))
            .padding(Padding::from([0, 10]))
            .align_y(Vertical::Center)
            .width(Length::Fill),
        )
        .center(Length::Fill)
        .height(40)
        .style(container::rounded_box)
        .into()
    }
}
