use std::io::Write;
use std::path::PathBuf;
use std::{fs, io::Read};

use directories::ProjectDirs;
use ggez::{glam::Vec2, graphics};
use serde::{Deserialize, Serialize};

pub const BACKGROUND_IMAGE: &[u8] = include_bytes!("../resources/background_tower_1.png");
pub const QUAS: &[u8] = include_bytes!("../resources/quas.png");
pub const WEX: &[u8] = include_bytes!("../resources/wex.png");
pub const EXORT: &[u8] = include_bytes!("../resources/exort.png");

#[derive(Debug, Clone)]
pub struct Settings {
    pub window_width: f32,
    pub window_height: f32,
    pub background_draw_param: graphics::DrawParam,
    pub font_size: f32,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let user_settings = UserSettings::new()?;
        Ok(Settings::from(user_settings))
    }
}

impl From<UserSettings> for Settings {
    fn from(user_settings: UserSettings) -> Self {
        let font_size = user_settings.window_height * 0.03;

        let scale_w = user_settings.window_width / 1024.0;
        let scale_y = user_settings.window_height / 1024.0;
        let scale = Vec2::new(scale_w, scale_y);
        let background_draw_param = graphics::DrawParam::new().scale(scale);

        Self {
            window_width: user_settings.window_width,
            window_height: user_settings.window_height,
            background_draw_param,
            font_size,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct UserSettings {
    window_width: f32,
    window_height: f32,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            window_width: 1024.0,
            window_height: 1024.0,
        }
    }
}

impl UserSettings {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_settings_path();
        println!("Settings path is {}", path.to_str().unwrap());
        if std::path::Path::exists(&path) {
            Ok(Self::load()?)
        } else {
            let settings = Self::default();
            settings.save()?;
            Ok(settings)
        }
    }

    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_settings_path();
        let mut file = fs::File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // If the toml is in the wrong format fall back to default
        match toml::from_str(&contents) {
            Ok(settings) => Ok(settings),
            Err(_) => Ok(Self::default()),
        }
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string(self)?;
        let mut file = fs::File::create(Self::get_settings_path())?;
        Ok(file.write_all(&contents.as_bytes())?)
    }

    fn get_settings_path() -> PathBuf {
        let project_dirs =
            ProjectDirs::from("", "", "Invoke").expect("Home directory must be present");

        let config_dir = project_dirs.config_dir();
        fs::create_dir_all(config_dir).expect("Should be able to create dirs");
        config_dir.join("settings.toml")
    }
}
