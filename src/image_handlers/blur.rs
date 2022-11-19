use crate::utils::save_copy;
use image::imageops::blur;
use image::open;
use inquire::Select;
use spinners::{Spinner, Spinners};

pub fn handle_blur(image_path: String) {
    let img = open(image_path.clone()).unwrap_or_else(|_| panic!("Unable to open image"));

    let options = vec!["low", "medium", "high", "ultra"];
    let blur_intensity = Select::new("Select blur intensity", options)
        .prompt()
        .unwrap_or_else(|e| panic!("blur failure: {}", e));

    let blur_value = match blur_intensity {
        "low" => 2.0,
        "medium" => 5.0,
        "high" => 10.0,
        "ultra" => 20.0,
        _ => panic!("Invalid blur value"),
    };

    let mut sp = Spinner::new(Spinners::Smiley, "Blurring your image -- This might take a while...".into());
    let new_image = blur(&img, blur_value);
    sp.stop();
    save_copy(image_path, new_image);
}
