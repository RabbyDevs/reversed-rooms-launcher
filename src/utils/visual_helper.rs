use std::{io::{Cursor, Write}, sync::Arc};

use file_format::FileFormat;
use iced_video_player::Video;
use ::image::ImageReader;
use iced::{widget::{container, image}, Element, Length};
use tempfile::NamedTempFile;

use crate::{Assets, LauncherBackground, Message, PossibleGames, State};

use super::img_utils::round_image;

pub fn get_game_background(state: &State) -> LauncherBackground {
    let file_path: &str = match state.selected_game {
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
                    LauncherBackground::Video(video)
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
            LauncherBackground::Image(image::Handle::from_rgba(
                img.width(), 
                img.height(), 
                img.to_rgba8().into_raw()
            ))
        }

    } else {
        panic!("Missing icon for {:?}, path: {}", state.selected_game, file_path)
    }
}

pub fn get_game_icon_handle(game: &PossibleGames) -> image::Handle {
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
        
        image::Handle::from_rgba(
            img.width(), 
            img.height(), 
            img.to_rgba8().into_raw()
        )
    } else {
        panic!("Missing icon for {:?}, path: {}", game, file_path)
    }
}

pub fn get_game_icon<'a>(state: &'a State, game: &'a PossibleGames) -> Element<'a, Message> {
    let handle = state.icon_images.get(game).unwrap();
    container(image(handle).content_fit(iced::ContentFit::Contain).height(Length::Fixed(64.0)).filter_method(image::FilterMethod::Linear)).into()
}