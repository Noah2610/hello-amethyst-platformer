use amethyst::utils::application_root_dir;

const RESOURCES_DIRNAME: &str = "resources";

pub fn resources_dir() -> String {
    format!("{}/{}", application_root_dir(), RESOURCES_DIRNAME)
}

pub fn resource<T: ToString>(path: T) -> String {
    format!("{}/{}", resources_dir(), path.to_string())
}
