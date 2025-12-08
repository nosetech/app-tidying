use log::LevelFilter;

pub fn init() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .try_init()
        .ok();
}
