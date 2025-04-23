use iced::widget::{button, canvas, column, container, row, text};
use iced::{Element, Fill, Theme};

pub fn main() -> iced::Result {
    iced::application("A counter", ZappyGui::update, ZappyGui::view)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TabSelected(Tab),
    IncrementCounter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Canvas,
    Settings,
    Info,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Canvas
    }
}

#[derive(Default)]
struct ZappyGui {
    active_tab: Tab,
    circle_state: CircleState
}

impl ZappyGui {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
            }
            Message::IncrementCounter => {
                self.circle_state.counter += 10;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let tab_canvas = button("Canvas")
            .on_press(Message::TabSelected(Tab::Canvas))
            .style(button::primary);

        let tab_settings = button("Paramètres")
            .on_press(Message::TabSelected(Tab::Settings))
            .style(button::primary);

        let tab_info = button("Info")
            .on_press(Message::TabSelected(Tab::Info))
            .style(button::primary);

        let idk = button("idk")
            .on_press(Message::IncrementCounter)
            .style(button::primary);

        let circle_canvas = canvas(Circle { radius: self.circle_state.counter as f32 })
            .width(200)
            .height(200);

        // Contenu basé sur l'onglet actif

        let content = match self.active_tab {
            Tab::Canvas => {
                column![container(circle_canvas)
                    .width(Fill)
                    .style(container::bordered_box)]
            }
            Tab::Info => {
                column![text("Information sur l'application")]
            }
            _ => {
                column![text("Paramètres de l'application")]
            }
        };

        // Structure finale
        column![
            row![tab_canvas, tab_settings, tab_info, idk].spacing(10),
            content
        ]
        .into()
    }
}

use iced::mouse;
use iced::{Color, Rectangle, Renderer};

// First, we define the data we need for drawing
#[derive(Debug)]
struct Circle {
    radius: f32,
}

#[derive(Debug, Default, Clone)]
struct CircleState {
    counter: u32,
}

// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for Circle {
    type State = CircleState;

    fn draw(
        &self,
        state: &CircleState,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Dessiner le cercle
        let circle = canvas::Path::circle(frame.center(), self.radius);
        frame.fill(&circle, Color::BLACK);

        // Afficher le compteur
        let text = format!("Compteur: {}", self.radius);
        vec![frame.into_geometry()]
    }
}

impl Circle {
    fn increment(&mut self) {
        self.radius += 1.0;
    }
}