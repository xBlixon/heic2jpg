use std::{fs};
use std::fs::{read_dir};
use std::path::Path;
use image::{DynamicImage, ImageFormat, ImageReader, Rgb, RgbImage};
use libheif_rs::{RgbChroma, ColorSpace, HeifContext, Result, LibHeif, Plane};
use libheif_rs::integration::image::register_all_decoding_hooks;
use zip::ZipArchive;

fn convert_all(source: &String, destination: &String) -> () {
    let source_dir = Path::new(source);
    let destination_dir = Path::new(destination);

    read_dir(source_dir).unwrap().for_each(|dir_file_result| {
        let dir_file = dir_file_result.unwrap();
        if !dir_file.file_type().unwrap().is_dir() {
            return;
        }

        read_dir(dir_file.path()).unwrap().for_each(|image_file_result| {
            let image_file = image_file_result.unwrap();
            let image_path = image_file.path().display().to_string();

            let jpeg = convert_to_jpeg(&image_path);
            let mut jpeg_filename = image_file.path().file_stem().unwrap().display().to_string();
            jpeg_filename.push_str(".jpg");
            jpeg.save_with_format(destination_dir.join(jpeg_filename), ImageFormat::Jpeg).unwrap();
            println!("CONVERTED!");
        });
    })
}

fn convert_to_jpeg(filename: &String) -> DynamicImage {
    println!("\n\nCONVERTING FILE: {}", filename);
    let reader = ImageReader::open(filename).unwrap();
    let image = reader.decode().unwrap();

    image
}
fn extract_files(dir: &String) -> () {

    let path = Path::new(dir.as_str());
    read_dir(path).unwrap().for_each(|entry_result| {
        let entry = entry_result.unwrap();
        println!("{:?}", entry.file_name());
        if !entry.file_type().unwrap().is_file() {
            return;
        }

        let zip_path = &entry.path().display().to_string();
        let zip_file = fs::File::open(&zip_path).unwrap();
        let mut archive = ZipArchive::new(zip_file).unwrap();
        let mut extract_path = dir.clone();
        let file_stem = entry.path().file_stem().unwrap().to_str().unwrap().to_owned();
        extract_path.push('\\');
        extract_path.push_str(&file_stem);

        println!("Extracting {}", entry.path().display());
        archive.extract(extract_path).unwrap();
    });


    read_dir(path).unwrap().for_each(|entry_result| {
        let entry = entry_result.unwrap();
        if entry.file_type().unwrap().is_dir() {
            if entry.file_name().to_str().unwrap().ends_with("_extracted") {
                return;
            }

            let destination_path = entry.path().parent().unwrap().to_owned();

            read_dir(entry.path()).unwrap().for_each(|entry_result| {
                let sub_entry = entry_result.unwrap();

                // let mut nested_path = entry.path().display().to_string();
                // nested_path.push_str("\\");
                // nested_path.push_str(entry.file_name().to_str().unwrap());
                let mut sub_dir = sub_entry.file_name().into_string().unwrap();
                sub_dir.push_str("_extracted");

                let destpath = destination_path.join(sub_dir);

                // destpath.push_str("_extracted");

                match fs::rename(sub_entry.path(), destpath) {
                    Ok(_) => (),
                    Err(..) => {
                        println!("Failed to extract sub directory: {}", sub_entry.path().display());
                    }
                };
            });
            fs::remove_dir_all(entry.path()).unwrap();

        }
    });
}

fn main() -> Result<()> {
    register_all_decoding_hooks();
    let source_dir: String = String::from("files");
    let destination_dir: String = String::from("converted");

    // extract_files(&source_dir);

    convert_all(&source_dir, &destination_dir);


    Ok(())
}