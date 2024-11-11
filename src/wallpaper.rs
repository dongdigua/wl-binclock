#[cfg(test)]
mod tests {
    use std::fs::File;

    use water_bg_config::Config;
    use xdg::BaseDirectories;

    #[test]
    fn test() {
        let xdg_dir = BaseDirectories::new().unwrap();
        let cache_img = xdg_dir
            .create_cache_directory("my_wl_app/test1.jpg")
            .unwrap();
        File::create(cache_img).unwrap();
    }

    #[test]
    fn test2() {
        let config = Config::default();
        let file = config.get_current_image();
        println!("len:{}", file.metadata().unwrap().len())
    }
}
