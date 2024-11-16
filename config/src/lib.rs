use std::{
    collections::HashMap,
    fs::{self, DirBuilder, File},
    io::Write,
    path::{Path, PathBuf},
    thread::{self, JoinHandle},
};

pub enum Day {
    Today,
}

impl Day {
    pub fn to_value(&self) -> u32 {
        match self {
            Day::Today => 0,
        }
    }
}

pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        let config = Self {};
        config.init();
        return config;
    }
}

impl Config {
    fn init(&self) {
        let path = Self::get_image_dir();
        if !path.exists() {
            DirBuilder::new().recursive(true).create(path).unwrap();
        }
    }

    fn get_image_dir() -> PathBuf {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("water-bg").unwrap();
        xdg_dirs.get_data_home()
    }

    pub fn get_current_image(&self) -> String {
        let date_time = chrono::offset::Local::now();
        let date = format!("{}", date_time.format("%Y%m%d"));
        if let Ok(entries) = fs::read_dir(Self::get_image_dir()) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.file_name().unwrap().to_str().unwrap().contains(&date)
                    {
                        return String::from(path.to_str().unwrap());
                    }
                }
            }
        }

        let handle = self.get_http_image(Day::Today);
        let file_path = handle.join().unwrap();
        return file_path;
    }

    fn get_http_image(&self, day: Day) -> JoinHandle<String> {
        thread::spawn(move || {
            let image_url = format!(
                "https://bing.biturl.top/?resolution=UHD&format=json&index={}&mkt=zh-CN",
                day.to_value()
            );
            let json = reqwest::blocking::get(image_url)
                .unwrap()
                .json::<HashMap<String, String>>()
                .unwrap();
            let image_url = json.get("url").unwrap();
            let end_date = json.get("end_date").unwrap();
            let sufix = image_url.split(".").last().unwrap();
            let path_str = format!("{}/{}.{}", Self::get_image_dir().display(), end_date, sufix);
            let file_path = Path::new(&path_str);
            if !file_path.exists() {
                let b = reqwest::blocking::get(image_url).unwrap().bytes().unwrap();
                let mut file = File::create(file_path).unwrap();
                file.set_len(b.len() as u64).unwrap();
                file.write_all(&b).unwrap();
            }
            return file_path.to_str().unwrap().to_string();
        })
    }
}
