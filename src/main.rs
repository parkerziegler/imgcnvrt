extern crate image;

use std::env;
use std::process;
use std::path::Path;
use image::GenericImageView;
use imgcnvrt::Config;

fn main() {
    let args = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let in_path = Path::new(&args.in_file_path);

    // Load the image from the input file path.
    // The path is guaranteed to be valid at this point based on the checks in Config::new.
    let img = image::open(in_path).unwrap();

    // Print the image's dimensions and ColorType.
    println!("Dimensions: {:?}", img.dimensions());
    println!("Color space: {:?}", img.color());

    // Construct the output file path.
    let out_file_path = match (in_path.parent(), in_path.file_stem()) {
        (Some(dir), Some(file_stem)) => 
            format!("{}/{}{}", dir.to_str().unwrap_or_default(), file_stem.to_str().unwrap_or_default(), &args.output_extension),
        _ => {
            eprintln!("Could not decode file location.");
            process::exit(1);
        }
    };

    img.save(Path::new(&out_file_path)).unwrap_or_else(|err| {
        eprintln!("Problem converting image: {}", err)
    });
}