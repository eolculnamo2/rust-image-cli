use image::imageops::resize;
use image::imageops::FilterType;
use image::open;
use image::GenericImageView;
use inquire::Select;
use spinners::{Spinner, Spinners};

fn create_copy_path(orignal_path: String) -> String {
    let dot_idx = orignal_path.rfind('.');
    match dot_idx {
        Some(idx) => {
            let mut new_path = orignal_path;
            new_path.insert_str(idx as usize, "-copy");
            new_path
        }
        None => panic!("invalid path. unable to find . for file extension"),
    }
}

pub fn handle_resize(image_path: String) {
    if let Ok(img) = open(image_path.clone()) {
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

        let mut sp = Spinner::new(Spinners::Dots9, "Saving your newly modified image...".into());
        let save_location = create_copy_path(image_path);
        match new_image.save(save_location) {
            Ok(_) => {
                sp.stop();
                println!("\nGreat Success!");
            }
            Err(e) => panic!("Save failure: {}", e),
        }
    } else {
        panic!("ahh noooooo");
    }
}
