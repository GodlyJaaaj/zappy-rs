use crate::game::GameState;
use alignment::Vertical;
use iced::widget::canvas::Cache;
use iced::widget::{Checkbox, Column, Container, Stack, Text, canvas, scrollable};
use iced::{Color, Element, Length, Padding, Pixels, Point, Rectangle, Vector, alignment};
use iced::{Size, mouse};
use iced_futures::core::alignment::Horizontal;
use std::rc::Rc;

pub struct MapView {
    min_tile_size: f32,
    max_tile_size: f32,
    zoom_level: f32,
    offset: Point,
    drag_start: Option<Point>,
    cache: Rc<Cache>,

    // Right panel
    show_coordinates: bool,
}

#[derive(Debug, Clone)]
pub enum MapMessage {
    Zoom(f32),
    ZoomIn,
    ZoomOut,
    DragStart(Point),
    DragTo(Point),
    DragEnd,
    ResetZoom,
    ToggleCoordinates(bool),
}

impl Default for MapView {
    fn default() -> Self {
        Self {
            min_tile_size: 10.0,
            max_tile_size: 100.0,
            zoom_level: 1.0,
            offset: Point::new(0.0, 0.0),
            drag_start: None,
            cache: Cache::new().into(),
            show_coordinates: false,
        }
    }
}

impl MapView {
    pub fn reset_zoom(&mut self) {
        let default = Self::default();
        self.zoom_level = default.zoom_level;
        self.offset = default.offset;
    }

    pub fn update(&mut self, message: MapMessage) {
        match message {
            MapMessage::Zoom(delta) => {
                self.zoom_level = (self.zoom_level * delta).max(0.1).min(5.0);
            }
            MapMessage::ZoomIn => {
                self.cache.clear();
                self.zoom_level = (self.zoom_level * 1.1).min(5.0);
            }
            MapMessage::ZoomOut => {
                self.cache.clear();
                self.zoom_level = (self.zoom_level * 0.9).max(0.1);
            }

            MapMessage::DragStart(position) => {
                self.drag_start = Some(position);
            }
            MapMessage::DragTo(position) => {
                self.cache.clear();
                if let Some(start) = self.drag_start {
                    let delta = Vector::new(position.x - start.x, position.y - start.y);
                    self.offset = Point::new(self.offset.x + delta.x, self.offset.y + delta.y);
                    self.drag_start = Some(position);
                }
            }
            MapMessage::DragEnd => {
                self.drag_start = None;
            }
            MapMessage::ResetZoom => {
                self.cache.clear();
                self.reset_zoom();
            }
            MapMessage::ToggleCoordinates(show) => {
                self.cache.clear();
                self.show_coordinates = show;
            }
        }
    }

    pub fn view<'a>(&self, game_state: &'a GameState) -> Element<'a, MapMessage> {
        if game_state.width().is_none() || game_state.width().is_none() {
            return Container::new(Text::new("En attente des dimensions de la map..."))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into();
        }

        let grid = canvas::Canvas::new(GridCanvas {
            game_state,
            min_tile_size: self.min_tile_size,
            max_tile_size: self.max_tile_size,
            zoom_level: self.zoom_level,
            offset: self.offset,
            show_coordinates: self.show_coordinates,
            cache: Rc::clone(&self.cache),
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let grid_container = Container::new(grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        use iced::widget::{Row, button};

        let reset_button = button(Text::new("Reset Zoom").size(14.0))
            .on_press(MapMessage::ResetZoom)
            .padding(
                Padding::default()
                    .left(35.0)
                    .right(35.0)
                    .top(5.0)
                    .bottom(5.0),
            );

        let zoom_in_button = button(Text::new("+")).on_press(MapMessage::ZoomIn);

        let zoom_out_button = button(Text::new("-")).on_press(MapMessage::ZoomOut);

        let show_coordinates_checkbox = Checkbox::new("Show Coordinates", self.show_coordinates)
            .on_toggle(MapMessage::ToggleCoordinates)
            .text_size(14.0);

        let panel_content = scrollable(
            Column::new()
                .push(reset_button)
                .push(show_coordinates_checkbox)
                .spacing(10)
                .padding(20)
                .align_x(alignment::Horizontal::Center),
        );

        let right_panel = Container::new(panel_content)
            .width(Length::Fixed(200.0))
            .height(Length::Fill);

        let zoom_dezoom_buttons = Row::new()
            .push(zoom_in_button)
            .push(zoom_out_button)
            .spacing(10)
            .padding(20);
        let zoom_dezoom_buttons = Container::new(zoom_dezoom_buttons)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
            .align_x(Horizontal::Right);

        let content = Row::new()
            .push(Stack::new().push(grid_container).push(zoom_dezoom_buttons))
            .push(right_panel)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct GridCanvas<'a> {
    game_state: &'a GameState,
    min_tile_size: f32,
    max_tile_size: f32,
    zoom_level: f32,
    offset: Point,
    show_coordinates: bool,
    cache: Rc<Cache>,
}

impl<'a> GridCanvas<'a> {
    fn draw_grid(&self, frame: &mut canvas::Frame, bounds: Rectangle, tile_size: f32) {
        let width = self.game_state.width().unwrap();
        let height = self.game_state.height().unwrap();

        let grid_width = width as f32 * tile_size;
        let grid_height = height as f32 * tile_size;

        let offset_x = (bounds.width - grid_width) / 2.0 + self.offset.x;
        let offset_y = (bounds.height - grid_height) / 2.0 + self.offset.y;

        // Remplissage de l'arrière-plan
        frame.fill(
            &canvas::Path::rectangle(Point::new(0.0, 0.0), Size::new(bounds.width, bounds.height)),
            Color::from_rgb(0.9, 0.9, 0.9),
        );

        // Dessiner les cases de la grille
        for y in 0..height {
            for x in 0..width {
                let x_pos = offset_x + x as f32 * tile_size;
                let y_pos = offset_y + y as f32 * tile_size;

                if x_pos + tile_size >= 0.0
                    && x_pos <= bounds.width
                    && y_pos + tile_size >= 0.0
                    && y_pos <= bounds.height
                {
                    let cell_color = if (x + y) % 2 == 0 {
                        Color::from_rgb(0.85, 0.85, 0.9)
                    } else {
                        Color::from_rgb(0.8, 0.8, 0.85)
                    };

                    frame.fill(
                        &canvas::Path::rectangle(
                            Point::new(x_pos, y_pos),
                            Size::new(tile_size, tile_size),
                        ),
                        cell_color,
                    );
                }
            }
        }

        // Dessiner les lignes de la grille
        let grid_color = Color::from_rgb(0.5, 0.5, 0.6);

        for y in 0..=height {
            let y_pos = offset_y + y as f32 * tile_size;

            if y_pos >= 0.0 && y_pos <= bounds.height {
                frame.stroke(
                    &canvas::Path::line(
                        Point::new(offset_x.max(0.0), y_pos),
                        Point::new((offset_x + grid_width).min(bounds.width), y_pos),
                    ),
                    canvas::Stroke::default()
                        .with_color(grid_color)
                        .with_width(1.0),
                );
            }
        }

        for x in 0..=width {
            let x_pos = offset_x + x as f32 * tile_size;

            if x_pos >= 0.0 && x_pos <= bounds.width {
                frame.stroke(
                    &canvas::Path::line(
                        Point::new(x_pos, offset_y.max(0.0)),
                        Point::new(x_pos, (offset_y + grid_height).min(bounds.height)),
                    ),
                    canvas::Stroke::default()
                        .with_color(grid_color)
                        .with_width(1.0),
                );
            }
        }

        // Afficher les coordonnées (si activé)
        if tile_size >= 20.0 && self.show_coordinates {
            for y in 0..height {
                for x in 0..width {
                    let x_pos = offset_x + x as f32 * tile_size;
                    let y_pos = offset_y + y as f32 * tile_size;

                    if x_pos + tile_size >= 0.0
                        && x_pos <= bounds.width
                        && y_pos + tile_size >= 0.0
                        && y_pos <= bounds.height
                    {
                        let center_x = x_pos + tile_size / 2.0;
                        let center_y = y_pos + tile_size / 2.0;

                        let text = format!("{},{}", x, y);

                        frame.fill_text(canvas::Text {
                            content: text,
                            position: Point::new(center_x, center_y),
                            color: Color::BLACK,
                            size: Pixels::from(tile_size * 0.3),
                            horizontal_alignment: Horizontal::Center,
                            vertical_alignment: Vertical::Center,
                            ..canvas::Text::default()
                        });
                    }
                }
            }
        }
    }
}

impl<'a> canvas::Program<MapMessage> for GridCanvas<'a> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<MapMessage>) {
        let is_over_canvas = cursor
            .position()
            .map_or(false, |position| bounds.contains(position));

        if !is_over_canvas {
            return (canvas::event::Status::Ignored, None);
        }

        match event {
            canvas::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::WheelScrolled { delta } => {
                    let zoom_factor = match delta {
                        mouse::ScrollDelta::Lines { y, .. }
                        | mouse::ScrollDelta::Pixels { y, .. } => {
                            if y > 0.0 {
                                1.1
                            } else if y < 0.0 {
                                0.9
                            } else {
                                1.0
                            }
                        }
                    };

                    if zoom_factor != 1.0 {
                        return (
                            canvas::event::Status::Captured,
                            Some(MapMessage::Zoom(zoom_factor)),
                        );
                    }
                }
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if let Some(position) = cursor.position() {
                        return (
                            canvas::event::Status::Captured,
                            Some(MapMessage::DragStart(position)),
                        );
                    }
                }
                mouse::Event::CursorMoved { position } => {
                    return (
                        canvas::event::Status::Captured,
                        Some(MapMessage::DragTo(position)),
                    );
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    return (canvas::event::Status::Captured, Some(MapMessage::DragEnd));
                }
                _ => {}
            },
            _ => {}
        }

        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let tile_size = self.zoom_level
                * self.min_tile_size.max(
                    (bounds.width.min(bounds.height)
                        / self.game_state.width().max(Some(1)).unwrap() as f32)
                        .min(self.max_tile_size),
                );
            self.draw_grid(frame, bounds, tile_size);
        });

        vec![geometry]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Grab
        } else {
            mouse::Interaction::default()
        }
    }
}
