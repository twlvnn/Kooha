use gsettings_macro::gen_settings;
use gtk::{
    gio::{self, prelude::*},
    glib,
};

use std::path::{Path, PathBuf};

use crate::config::APP_ID;

#[gen_settings(file = "./data/io.github.seadve.Kooha.gschema.xml.in")]
#[gen_settings_skip(key_name = "saving-location")]
pub struct Settings;

impl Default for Settings {
    fn default() -> Self {
        Self::new(APP_ID)
    }
}

impl Settings {
    pub fn set_saving_location(&self, directory: &Path) {
        self.0
            .set_string("saving-location", directory.to_str().unwrap())
            .unwrap();
    }

    pub fn saving_location(&self) -> PathBuf {
        let saving_location = self.0.string("saving-location").to_string();

        if saving_location == "default" {
            glib::user_special_dir(glib::UserDirectory::Videos).unwrap_or_else(glib::home_dir)
        } else {
            PathBuf::from(saving_location)
        }
    }

    pub fn file_path(&self) -> PathBuf {
        let file_name = glib::DateTime::now_local()
            .expect("You are somehow on year 9999")
            .format("Kooha-%F-%H-%M-%S") // TODO improve format
            .expect("Invalid format string")
            .to_string();

        let mut path = self.saving_location();
        path.push(file_name);
        path.set_extension(&self.video_format().to_variant().get::<String>().unwrap());
        path
    }
}