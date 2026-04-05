use std::fs::{create_dir_all, read_dir, remove_dir_all, remove_file};
use std::path::{absolute};
use image::{DynamicImage, ImageFormat, ImageReader};
use libheif_rs::{Result};
use libheif_rs::integration::image::register_all_decoding_hooks;
use chrono::{Local};

mod extract;

fn convert_all(source: &String, destination: &String) -> () {
    let source_dir = absolute(source).unwrap();
    let destination_dir = absolute(destination).unwrap();

    read_dir(source_dir).unwrap().for_each(|unzip_file_result| {
       let unzip_file = unzip_file_result.unwrap();
        if !unzip_file.file_type().unwrap().is_dir() {
            return;
        }


        read_dir(unzip_file.path()).unwrap().for_each(|dir_file_result| {
            let dir_file = dir_file_result.unwrap();

            let dir_name = dir_file.file_name().to_str().unwrap().to_owned();
            let mut dated_dir_name = today_directory();
            dated_dir_name.push('_');
            dated_dir_name.push_str(dir_name.as_str());

            let final_dir = destination_dir.join(dated_dir_name);

            match create_dir_all(final_dir.to_str().unwrap()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Directory creation failed! : {}", e);
                },
            }

            read_dir(dir_file.path()).unwrap().for_each(|image_file_result| {
                let image_file = image_file_result.unwrap();
                let image_path = image_file.path().display().to_string();

                let jpeg = convert_to_jpeg(&image_path);
                let mut jpeg_filename = image_file.path().file_stem().unwrap().display().to_string();
                jpeg_filename.push_str(".jpg");
                match jpeg.save_with_format(final_dir.join(jpeg_filename), ImageFormat::Jpeg) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Failed to save HEIC in JPEG: {}", e);
                    }
                }
                println!("CONVERTED!");
            });
        })
    });

}

fn convert_to_jpeg(filename: &String) -> DynamicImage {
    println!("\nCONVERTING FILE: {}", filename);
    let reader = ImageReader::open(filename).unwrap();
    let image = reader.decode().unwrap();

    image
}

fn today_directory() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d").to_string()
}

fn wipe_source_dir(source_dir: &String) -> () {
    let path = absolute(source_dir).unwrap();

    read_dir(path).unwrap().for_each(|dir_file_result| {
        let dir_file = dir_file_result.unwrap();
        if dir_file.file_type().unwrap().is_dir() {
            remove_dir_all(dir_file.path()).unwrap();
        } else {
            remove_file(dir_file.path()).unwrap();
        }
    })
}


fn main() -> Result<()> {
    register_all_decoding_hooks();
    let source_dir: String = String::from("src");
    let destination_dir: String = String::from("dest");

    extract::extract_files(&source_dir);

    convert_all(&source_dir, &destination_dir);

    wipe_source_dir(&source_dir);

    Ok(())
}