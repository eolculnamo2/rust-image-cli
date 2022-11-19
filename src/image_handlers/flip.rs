use image::imageops::{flip_vertical, flip_horizontal};
use image::open;
use inquire::Select;
use crate::utils::save_copy;

pub fn handle_flip(image_path: String) {
    let img = open(image_path.clone()).unwrap_or_else(|_| panic!("Unable to open image"));
    let options = vec!["horizontal", "vertical", "cancel"];
    let direction = Select::new("Select flip direction", options).prompt().unwrap_or_else(|_| panic!("Failed to select direction"));

    let flipped_img = match direction {
        "horizontal" => flip_horizontal(&img),
        "vertical" => flip_vertical(&img),
        _ => panic!("Canceling flip"),
    };
    save_copy(image_path, flipped_img);
}
