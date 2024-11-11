use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    thread::{self, JoinHandle},
};

const IMAGE_DIR: &str = "/usr/share/backgrounds/water-bg/";

enum Day {
    Today,
    Yesterday,
}

impl Day {
    fn to_value(&self) -> u32 {
        match self {
            Day::Today => 0,
            Day::Yesterday => 1,
        }
    }
}

pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        let config = Self {};
        config.get_http_image(Day::Yesterday);
        return config;
    }
}

impl Config {
    pub fn get_current_image(&self) -> File {
        let date_time = chrono::offset::Local::now();
        let date = format!("{}", date_time.format("%Y%m%d"));
        if let Ok(entries) = fs::read_dir(Path::new(IMAGE_DIR)) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.file_name().unwrap().to_str().unwrap().contains(&date)
                    {
                        println!("today file exists");
                        return File::open(path).unwrap();
                    }
                }
            }
        }

        let handle = self.get_http_image(Day::Today);
        let file_path = handle.join().unwrap();
        return File::open(file_path).unwrap();
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
            let path_str = format!("{}/{}.{}", IMAGE_DIR, end_date, sufix);
            let file_path = Path::new(&path_str);
            if !file_path.exists() {
                let b = reqwest::blocking::get(image_url).unwrap().bytes().unwrap();
                let mut file = File::create(file_path).unwrap();
                file.set_len(b.len() as u64).unwrap();
                file.write_all(&b).unwrap();
            } else {
                println!("file already exists")
            }
            return file_path.to_str().unwrap().to_string();
        })
    }
}
