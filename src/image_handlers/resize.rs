use image::imageops::resize;
use image::imageops::FilterType;
use image::open;
use image::GenericImageView;
use inquire::Select;

use crate::utils::save_copy;

pub fn handle_resize(image_path: String) {
    let img = open(image_path.clone())
        .unwrap_or_else(|_| panic!("Unable to find image at path provided"));
    let (width, height) = img.dimensions();
    let aspect_ratio = width as f32 / height as f32;

    let options = vec!["thumbnail", "small", "medium", "large"];
    let resize_option_result = Select::new("Select a size", options).prompt();
    let resize_option = match resize_option_result {
        Err(e) => panic!("Resize option response error {}", e),
        Ok(o) => o,
    };

    let user_input_width_selection = match resize_option {
        "thumbnail" => 100,
        "small" => 400,
        "medium" => 800,
        _ => 1080,
    };

    // do not exceed original width
    let user_input_width = if user_input_width_selection < width {
        user_input_width_selection
    } else {
        width
    };

    let derived_height = user_input_width as f32 / aspect_ratio as f32;

    let new_image = resize(
        &img,
        user_input_width,
        derived_height.round() as u32,
        FilterType::Nearest,
    );
    save_copy(image_path, new_image);
}
