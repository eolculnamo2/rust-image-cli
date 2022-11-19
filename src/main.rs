use clap::Parser;
use inquire::Select;

mod image_handlers;

use image_handlers::resize::handle_resize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    image_path: String,
}

fn main() {
    let args = Args::parse();

    let options = vec!["resize", "cancel"];
    let image_action = Select::new("Select an action", options).prompt();

    if let Ok(img_action) = image_action {
        match img_action {
            "resize" => handle_resize(args.image_path),
            _ => println!("canceling"),
        };
    }
}
