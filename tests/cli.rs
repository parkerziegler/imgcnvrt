use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::path::PathBuf;
// Used for writing assertions
use std::process::Command; // Run programs
use std::path::Path;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Failure {}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failure!")
    }
}

impl Error for Failure {}

fn normalize_file_path(folder: &str, img: &str) -> PathBuf {
    Path::new(folder).join(img)
}

#[test]
fn it_converts_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imgcnvrt")?;

    let input_path = normalize_file_path("tests", "fractal.png");
    let output_path = normalize_file_path("tests", "fractal.jpg");

    cmd.arg(input_path.as_os_str()).arg(".jpg");
    cmd.assert()
        .success();

    if output_path.is_file() {
        fs::remove_file(output_path).unwrap();

        Ok(())
    } else {
        Err(Box::new(Failure {}))
    }
}

#[test]
fn it_errors_if_no_file_specified() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imgcnvrt")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No input file path specified."));

    Ok(())
}

#[test]
fn it_errors_on_file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imgcnvrt")?;

    let input_path = normalize_file_path("tests", "test.jpg");

    cmd.arg(input_path.as_os_str()).arg(".png");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("The provided path does not pertain to a file on disk."));

    Ok(())
}

#[test]
fn it_errors_if_no_output_extension_specified() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imgcnvrt")?;

    let input_path = normalize_file_path("tests", "fractal.png");

    cmd.arg(input_path.as_os_str());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No output extension specified."));

    Ok(())
}

#[test]
fn it_errors_on_unsupported_extensions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imgcnvrt")?;

    let input_path = normalize_file_path("tests", "fractal.png");

    cmd.arg(input_path.as_os_str()).arg(".mp3");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported file extension."));

    Ok(())
}

#[test]
fn it_decodes_png_and_encodes_supported_image_formats() -> Result<(), Box<dyn std::error::Error>> {
    let supported_img_formats = vec![".jpg", ".jpeg", ".gif", ".bmp", ".tiff", ".pnm", ".tga", ".ff"];

    let mut failed = false;
    
    for img_format in supported_img_formats {
        let mut cmd = Command::cargo_bin("imgcnvrt")?;

        let input_path = normalize_file_path("tests", "fractal.png");

        cmd.arg(input_path.as_os_str()).arg(img_format);
        cmd.assert()
            .success();

        let output_path = normalize_file_path("tests", &format!("fractal{}", &img_format));

        if output_path.is_file() {
            fs::remove_file(output_path).unwrap();
        } else {
            failed = true;
        }
    }

    if failed {
        Err(Box::new(Failure {}))
    } else {
        Ok(())
    }
}
