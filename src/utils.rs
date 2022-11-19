use image::{ImageBuffer, Rgba};
use spinners::{Spinner, Spinners};

pub fn create_copy_path(orignal_path: String) -> String {
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
pub fn save_copy(image_path: String, new_image: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut sp = Spinner::new(
        Spinners::Dots9,
        "Saving your newly modified image...".into(),
    );
    let save_location = create_copy_path(image_path);
    match new_image.save(save_location) {
        Ok(_) => {
            sp.stop();
            println!("\nGreat Success!");
        }
        Err(e) => panic!("Save failure: {}", e),
    }
}
