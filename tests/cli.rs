use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
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

#[test]
fn it_converts_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imgcnvrt")?;

    cmd.arg("tests/fractal.png").arg(".jpg");
    cmd.assert()
        .success();

    let path = Path::new("tests/fractal.jpg");

    if path.is_file() {
        fs::remove_file(path).unwrap();

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

    cmd.arg("tests/test.jpg").arg(".png");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("The provided path does not pertain to a file on disk."));

    Ok(())
}

#[test]
fn it_errors_if_no_output_extension_specified() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imgcnvrt")?;

    cmd.arg("tests/fractal.png");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No output extension specified."));

    Ok(())
}

#[test]
fn it_errors_on_unsupported_extensions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("imgcnvrt")?;

    cmd.arg("tests/fractal.png").arg(".mp3");
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

        cmd.arg("tests/fractal.png").arg(img_format);
        cmd.assert()
            .success();

        let out_path = format!("tests/fractal{}", &img_format);
        let path = Path::new(&out_path);

        if path.is_file() {
            fs::remove_file(path).unwrap();
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
