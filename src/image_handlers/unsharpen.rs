use crate::utils::save_copy;
use image::imageops::unsharpen;
use image::open;
use inquire::Select;
use spinners::{Spinner, Spinners};

pub fn handle_unsharpen(image_path: String) {
    let img = open(image_path.clone()).unwrap_or_else(|_| panic!("Unable to open image"));

    let options = vec!["low", "medium", "high", "ultra"];
    let sigma_intensity  = Select::new("Select sigma", options)
        .prompt()
        .unwrap_or_else(|e| panic!("unsharpen failure: {}", e));

    let sigma_value = match sigma_intensity {
        "low" => 2.0,
        "medium" => 5.0,
        "high" => 10.0,
        "ultra" => 20.0,
        _ => panic!("Invalid unsharpen value"),
    };

    let options = vec!["low", "medium", "high", "ultra"];
    let threshold = Select::new("Select threshold", options)
        .prompt()
        .unwrap_or_else(|e| panic!("unsharpen failure: {}", e));

    let threshold_value = match threshold {
        "low" => 2,
        "medium" => 10,
        "high" => 20,
        "ultra" => 50,
        _ => panic!("Invalid unsharpen value"),
    };


    let mut sp = Spinner::new(Spinners::Smiley, "Unsharpening your image -- This might take a while...".into());
    let new_image = unsharpen(&img, sigma_value, threshold_value);
    sp.stop();
    save_copy(image_path, new_image);

}
