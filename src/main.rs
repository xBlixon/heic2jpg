use std::fs::{read_dir, ReadDir};
use std::path::Path;
use image::{ImageBuffer, Rgb, RgbImage};
use libheif_rs::{Channel, RgbChroma, ColorSpace, HeifContext, Result, ItemId, LibHeif, Plane};

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

fn main() -> Result<()> {
    let input_path = "sewing-threads.heic";
    let output_path = "output.jpg";

    println!("Opening \"{}\"...", input_path);
    let img = convert_to_jpeg(input_path.to_string());

    img.save(output_path).expect("Nie udało się zapisać pliku JPEG");

    println!("\"{}\" has been converted to {}", input_path, output_path);
    Ok(())
}