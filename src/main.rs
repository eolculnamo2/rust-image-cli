use clap::Parser;
use inquire::Select;

mod image_handlers;
mod utils;

use image_handlers::resize::handle_resize;
use image_handlers::flip::handle_flip;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    image_path: String,
}

fn main() {
    let args = Args::parse();

    let options = vec!["resize", "flip", "cancel"];
    let image_action = Select::new("Select an action", options).prompt();

    if let Ok(img_action) = image_action {
        match img_action {
            "resize" => handle_resize(args.image_path),
            "flip" => handle_flip(args.image_path),
            _ => println!("canceling"),
        };
    }
}
