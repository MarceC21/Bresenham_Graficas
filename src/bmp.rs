use raylib::prelude::*;
use std::fs::File;
use std::io::Write;

pub fn write_bmp(
    filename: &str,
    width: usize,
    height: usize,
    pixels: &[Color],
) {

    let mut file = File::create(filename).unwrap();

    let filesize = 54 + width * height * 4;

    let mut header = [0u8; 54];

    header[0] = b'B';
    header[1] = b'M';

    header[2..6].copy_from_slice(&(filesize as u32).to_le_bytes());

    header[10] = 54;

    header[14] = 40;

    header[18..22].copy_from_slice(&(width as u32).to_le_bytes());

    header[22..26].copy_from_slice(&(height as u32).to_le_bytes());

    header[26] = 1;

    header[28] = 32;

    file.write_all(&header).unwrap();

    // Escribir los píxeles desde abajo hacia arriba
    for y in (0..height).rev() {

        for x in 0..width {

            let color = pixels[y * width + x];

            file.write_all(&[
                color.b,
                color.g,
                color.r,
                color.a,
            ]).unwrap();

        }

    }

}