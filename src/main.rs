use std::fs;
use std::fs::{read_dir, ReadDir};
use std::iter::Zip;
use std::path::Path;
use image::{ImageBuffer, Rgb, RgbImage};
use libheif_rs::{Channel, RgbChroma, ColorSpace, HeifContext, Result, ItemId, LibHeif, Plane};
use zip::ZipArchive;

fn convert_to_jpeg(filename: String) -> RgbImage {
    let ctx = HeifContext::read_from_file(filename.as_str()).expect("Nie udało się odczytać pliku HEIC");
    let handle = ctx.primary_image_handle().expect("Plik HEIC nie zawiera głównego obrazu");

    let lib = LibHeif::new();
    let image = lib.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)
        .expect("Błąd podczas dekodowania obrazu");
    let width = handle.width();
    let height = handle.height();

    let planes = image.planes();
    let plane = planes.interleaved.expect("Brak przeplotu pikseli (interleaved channel)");

    let stride = plane.stride as u32;
    let data = plane.data;

    println!("\"{}\" read. Detected dimmensions: {} x {}", filename, width, height);
    println!("Converting to JPG...");
    paint_jpeg(&plane, width, height)
}

fn paint_jpeg(plane: &Plane<&[u8]>, width: u32, height: u32) -> RgbImage {
    let mut img_buf = RgbImage::new(width, height);
    let data = plane.data;
    let stride = plane.stride as u32;

    for y in 0..height {
        for x in 0..width {
            let offset = (y * stride + x * 3) as usize;
            let r = data[offset];
            let g = data[offset + 1];
            let b = data[offset + 2];
            img_buf.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    img_buf
}

fn extract_files(dir: String) -> () {

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
            let mut nested_path = entry.path().display().to_string();
            nested_path.push_str("\\");
            nested_path.push_str(entry.file_name().to_str().unwrap());

            let mut destination_path = entry.path().display().to_string();
            destination_path.push_str("_extracted");

            fs::rename(nested_path, destination_path).unwrap();
            fs::remove_dir(entry.path()).unwrap();
        }
    });
}

fn main() -> Result<()> {
    let input_path = "sewing-threads.heic";
    let output_path = "output.jpg";

    let source_dir: String = String::from("files");

    extract_files(source_dir);

    // println!("Opening \"{}\"...", input_path);
    // let img = convert_to_jpeg(input_path.to_string());

    // img.save(output_path).expect("Nie udało się zapisać pliku JPEG");

    // println!("\"{}\" has been converted to {}", input_path, output_path);
    Ok(())
}