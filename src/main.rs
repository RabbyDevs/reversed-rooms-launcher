use file_format::FileFormat;
use ::image::ImageReader;
use iced::{
    alignment::Vertical::Top, border, gradient, mouse, widget::{button, center, column, container, image, mouse_area, row, stack, text, Column, Space}, window::{self, icon, Settings}, Alignment::Center, Color, Element, Font, Length, Point, Renderer, Size, Subscription, Task, Theme
};
use iced_video_player::{Video, VideoPlayer};
use serde::{Deserialize, Serialize};
use utils::img_utils::round_image;
use std::{
    env, fs::{self, create_dir_all, read_to_string}, io::{Cursor, Read, Write}, path::PathBuf, sync::Arc
};
mod utils;
use tempfile::NamedTempFile;
use iced::event::{self, Event};
use iced::keyboard::Event as KeyboardEvent;
use iced::mouse::Event as MouseEvent;

#[derive(rust_embed::Embed)]
#[folder = "resources"]
struct Assets;

pub fn main() -> iced::Result {
    let icon_file = Assets::get("icon.png").unwrap();
    let icon_image = ImageReader::new(Cursor::new(icon_file.data))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let rgba_vec = icon_image.as_rgba8().unwrap().to_vec();

    let settings = Settings {
        decorations: false,
        icon: Some(icon::from_rgba(rgba_vec, icon_image.width(), icon_image.height()).unwrap()),
        size: Size::new(2000.0, 1000.0),
        maximized: false,
        fullscreen: false,
        position: window::Position::Centered,
        max_size: None,
        min_size: None,
        visible: true,
        resizable: true,
        transparent: false,
        level: window::Level::Normal,
        exit_on_close_request: true,
        ..Settings::default()
    };

    iced::application(Launcher::boot, Launcher::update, Launcher::view)
        .title(Launcher::title)
        .window(settings)
        .window_size((1280.0, 760.0))
        .run()
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
enum PossibleGames {
    #[default]
    WutheringWaves,
    ZenlessZoneZero,
    HonkaiStarRail,
    GenshinImpact,
}

#[derive(Debug)]
enum Launcher {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default, Clone)]
struct State {
    selected_game: PossibleGames,
    installed_games: Vec<PossibleGames>,
    installed_game_servers: Vec<PossibleGames>,
    db_software_installed: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SavedState {
    installed_games: Vec<PossibleGames>,
    installed_game_servers: Vec<PossibleGames>,
    db_software_installed: bool,
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug)]
enum SaveError {
    Write,
    Format,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<State, LoadError>),
    DragStarted,
    GameSelected(PossibleGames)
}

impl State {
    // fn path() -> PathBuf {
    //     path.push("launcher-state.json");

    //     path
    // }

    // fn load() -> Result<State, LoadError> {
    //     let contents = read_to_string(Self::path()).map_err(|_| LoadError::File)?;

    //     let saved_state: SavedState =
    //         serde_json::from_str(&contents).map_err(|_| LoadError::Format)?;

    //     Ok(State {
    //         selected_game: PossibleGames::WutheringWaves,
    //         installed_games: saved_state.installed_games,
    //         installed_game_servers: saved_state.installed_game_servers,
    //         db_software_installed: saved_state.db_software_installed,
    //     })
    // }

    // async fn save(self) -> Result<(), SaveError> {
    //     let saved_state = SavedState {
    //         installed_games: self.installed_games,
    //         installed_game_servers: self.installed_game_servers,
    //         db_software_installed: self.db_software_installed,
    //     };

    //     let json = serde_json::to_string_pretty(&saved_state).map_err(|_| SaveError::Format)?;

    //     let path = Self::path();

    //     if let Some(dir) = path.parent() {
    //         create_dir_all(dir).map_err(|_| SaveError::Write)?;
    //     }

    //     {
    //         fs::write(path, json.as_bytes()).map_err(|_| SaveError::Write)?;
    //     }

    //     sleep(std::time::Duration::from_secs(2));

    //     Ok(())
    // }
}

fn rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

fn get_game_background(game: &PossibleGames) -> Element<Message> {
    let file_path: &str = match game {
        PossibleGames::WutheringWaves => "wutheringwaves-bg.mp4",
        PossibleGames::ZenlessZoneZero => "zenlesszonezero-bg.png",
        PossibleGames::HonkaiStarRail => "honkaistarrail-bg.png",
        PossibleGames::GenshinImpact => "genshinimpact-bg.png",
    };

    if let Some(file) = Assets::get(file_path) {
        let data = Arc::new(file.data);
        let file_format = FileFormat::from_bytes(&*data);
        if file_format.extension() == "mp4" {
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.write_all(&data).unwrap();

            let temp_path = temp_file.path().to_str().unwrap().to_string();
            match Video::new(url::Url::from_file_path(temp_path).unwrap()) {
                Ok(mut video) => {
                    video.set_looping(true);

                    VideoPlayer::new(video).into()
                },
                Err(err) => {
                    panic!("{:#?}", err)
                },
            }
        } else {
            let img = ImageReader::new(Cursor::new(&*data))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            let handle = image::Handle::from_rgba(
                img.width(), 
                img.height(), 
                img.to_rgba8().into_raw()
            );
            image(handle).content_fit(iced::ContentFit::Fill).into()
        }

    } else {
        panic!("Missing icon for {:?}, path: {}", game, file_path)
    }
}

fn get_game_icon(game: &PossibleGames) -> Element<Message> {
    let file_path: &str = match game {
        PossibleGames::WutheringWaves => "wutheringwaves-icon.png",
        PossibleGames::ZenlessZoneZero => "zenlesszonezero-icon.png",
        PossibleGames::HonkaiStarRail => "honkaistarrail-icon.png",
        PossibleGames::GenshinImpact => "genshinimpact-icon.png",
    };
    if let Some(img_file) = Assets::get(file_path) {
        let data_cursor = Cursor::new(img_file.data);
        let img = round_image(data_cursor)
            .unwrap()
            .resize(126, 126, ::image::imageops::FilterType::Lanczos3);
        let handle = image::Handle::from_rgba(
            img.width(), 
            img.height(), 
            img.to_rgba8().into_raw()
        );
        container(image(handle).content_fit(iced::ContentFit::Contain).height(Length::Fixed(64.0)).filter_method(image::FilterMethod::Linear))
        .style(move |_| {
            container::Style {
                border: border::rounded(20),
                ..container::Style::default()
            }
        })
        .into()
    } else {
        panic!("Missing icon for {:?}, path: {}", game, file_path)
    }
}

fn style_container(direction: f32, use_gradient: bool) -> container::Style {
    let angle = rad(direction);
    let gradient: Option<iced::Background> = if use_gradient {            
        Some(gradient::Linear::new(angle)
        .add_stop(0.0, Color::from_rgba8(0, 0, 0, 0.0))
        .add_stop(1.0, Color::from_rgba8(0, 0, 0, 0.8)).into())
    } else {None};
    container::Style {
        text_color: Color::from_rgba8(255, 255, 255, 1.0).into(),
        background: gradient,
        ..container::Style::default()
    }
}
impl Launcher {
    fn boot() -> (Self, Task<Message>) {
        (Self::Loaded(State::default()), Task::none())
    }

    fn title(&self) -> String {
        format!("RR Launcher v{}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            Launcher::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = Launcher::Loaded(state);
                    Task::none()
                },
                _ => Task::none(),
            },
            Launcher::Loaded(_) => {
                match message {
                    Message::DragStarted => {
                        window::get_latest().and_then(move |id: window::Id| {
                            window::drag(id)
                        })
                    },
                    _ => Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let game_selector = container(
            row![
                get_game_icon(&PossibleGames::WutheringWaves),
                get_game_icon(&PossibleGames::ZenlessZoneZero),
                get_game_icon(&PossibleGames::HonkaiStarRail),
                get_game_icon(&PossibleGames::GenshinImpact),
            ]
            .spacing(10),
        )
        .padding(10)
        .align_y(Top)
        .align_x(Center)
        .width(Length::Fill)
        .style(move |_| style_container(0.0, true));
        
        println!("whuh");
        match self {
            Launcher::Loading => center(text("Loading...").size(50)).into(),
            Launcher::Loaded(state) => {
                let topbar = container(
                    mouse_area(row![
                    text("Reversed Rooms").size(25),
                    Space::new(Length::Fill, Length::Fixed(0.0)),
                ])
                .on_press(Message::DragStarted))
                .width(Length::Fill)
                .style(move |_| style_container(0.0, false))
                .padding(10);
        
                let bottom_bar = container(row![
                    text("insert game announcements").size(25),
                    Space::new(Length::Fill, Length::Fixed(0.0)),
                    container(mouse_area(button(text("Launch").size(25))
                        .padding(10)
                        .style(move |_, _| {
                            button::Style {
                                text_color: Color::from_rgba8(0, 0, 0, 1.0),
                                background: Some(Color::from_rgba8(255, 255, 255, 1.0).into()),
                                border: border::rounded(5),
                                ..button::Style::default()
                            }
                        })).interaction(iced::mouse::Interaction::Pointer))
                ])
                .width(Length::Fill)
                .style(move |_theme| style_container(180.0, true))
                .padding(20);
        
                let user_area: Column<Message, Theme, Renderer> =
                    column![topbar, Space::new(Length::Fill, Length::Fill), bottom_bar].width(Length::Fill);
        
                let content = container(user_area).center(Length::Fill);

                stack![get_game_background(&state.selected_game), game_selector, content].into()
            }
        }
    }
}

