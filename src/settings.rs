use std::io::Write;
use std::path::PathBuf;
use std::{fs, io::Read};

use directories::ProjectDirs;
use ggez::{glam::Vec2, graphics};
use serde::{Deserialize, Serialize};

use ggez::input::keyboard::KeyCode;

#[derive(Debug, Clone)]
pub struct Settings {
    pub window_width: f32,
    pub window_height: f32,
    pub background_draw_param: graphics::DrawParam,
    pub font_size: f32,
    pub quas_key: KeyCode,
    pub wex_key: KeyCode,
    pub exort_key: KeyCode,
    pub invoke_key: KeyCode,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let user_settings = UserSettings::new()?;
        Ok(Settings::from(user_settings))
    }

    fn string_to_keycode(key_mapping: String) -> KeyCode {
        match key_mapping.as_str() {
            "A" => KeyCode::A,
            "B" => KeyCode::B,
            "C" => KeyCode::C,
            "D" => KeyCode::D,
            "E" => KeyCode::E,
            "F" => KeyCode::F,
            "G" => KeyCode::G,
            "H" => KeyCode::H,
            "I" => KeyCode::I,
            "J" => KeyCode::J,
            "K" => KeyCode::K,
            "L" => KeyCode::L,
            "M" => KeyCode::M,
            "N" => KeyCode::N,
            "O" => KeyCode::O,
            "P" => KeyCode::P,
            "Q" => KeyCode::Q,
            "R" => KeyCode::R,
            "S" => KeyCode::S,
            "T" => KeyCode::T,
            "U" => KeyCode::U,
            "V" => KeyCode::V,
            "W" => KeyCode::W,
            "X" => KeyCode::X,
            "Y" => KeyCode::Y,
            "Z" => KeyCode::Z,
            _ => unreachable!(),
        }
    }
}

impl From<UserSettings> for Settings {
    fn from(user_settings: UserSettings) -> Self {
        let font_size = user_settings.window_height * 0.03;

        let scale_w = user_settings.window_width / 1024.0;
        let scale_y = user_settings.window_height / 1024.0;
        let scale = Vec2::new(scale_w, scale_y);
        let background_draw_param = graphics::DrawParam::new().scale(scale);

        let quas_key = Self::string_to_keycode(user_settings.quas_key);
        let wex_key = Self::string_to_keycode(user_settings.wex_key);
        let exort_key = Self::string_to_keycode(user_settings.exort_key);
        let invoke_key = Self::string_to_keycode(user_settings.invoke_key);

        Self {
            window_width: user_settings.window_width,
            window_height: user_settings.window_height,
            background_draw_param,
            font_size,
            quas_key,
            wex_key,
            exort_key,
            invoke_key,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct UserSettings {
    window_width: f32,
    window_height: f32,
    quas_key: String,
    wex_key: String,
    exort_key: String,
    invoke_key: String,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            window_width: 1024.0,
            window_height: 1024.0,
            quas_key: String::from("Q"),
            wex_key: String::from("W"),
            exort_key: String::from("E"),
            invoke_key: String::from("R"),
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
