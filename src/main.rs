use std::io;
use std::io::prelude::*;
use std::fs::File;

mod ines;

extern crate image;

fn main() -> io::Result<()> {
    let mut f = File::open("sample1/sample1.nes")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let data = ines::INes::parse_from(&buffer);


    // ImageBuffer
    let imgx = 256;
    let imgy = 240;
    let mut imgbuf = image::GrayImage::new(imgx, imgy);

    for sprite_idx in 0..(ines::CHR_SIZE * data.header.size_of_chr_rom_in_16kb as u16 / 16) {
        let pattern_head: usize = sprite_idx as usize * 16;
        // https://wiki.nesdev.com/w/index.php/PPU_pattern_tables
        let pattern_table = &data.character[pattern_head..pattern_head + 16];

        for y in 0..8 {
            for x in 0..8 {
                let mut e: u8 = 0;

                let first = pattern_table[y] >> x & 0x01;
                e += first;

                let second = pattern_table[y + 8] >> x & 0x01;
                e += second << 1;
                
                let px = (sprite_idx as u32 % 32) * 8 + 8 - x as u32;
                let py = (sprite_idx as u32 / 32) * 8 + y as u32;

                imgbuf.put_pixel(px, py, image::Luma([e * 64 as u8]));
                // println!("({}, {}) : {:?}", px, py, imgbuf.get_pixel(px, py));
            }
        }
    }

    imgbuf.save("sprite.png").unwrap();

    Ok(())
}
