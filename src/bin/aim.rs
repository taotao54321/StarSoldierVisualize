use image::{Rgba, RgbaImage};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    path_out: std::path::PathBuf,
}

fn aim((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u8 {
    const TABLE: [u8; 24] = [
        14, 2, 10, 6, 14, 2, 10, 6, 15, 1, 9, 7, 13, 3, 11, 5, 0, 0, 8, 8, 12, 4, 12, 4,
    ];

    let mut idx = 0;

    if x1 < x2 {
        idx |= 1 << 0;
    }
    let dx = (x1 - x2).abs();

    if y1 < y2 {
        idx |= 1 << 1;
    }
    let dy = (y1 - y2).abs();

    if dy < dx {
        idx |= 1 << 2;
    }
    let a = (dx - dy).abs() / 4;
    let b = dx.min(dy) / 4;

    if 3 * a >= b {
        if a + a / 4 >= b {
            idx |= 1 << 4;
        } else {
            idx |= 1 << 3;
        }
    }

    TABLE[idx]
}

fn visualize() -> RgbaImage {
    const COLORS: [Rgba<u8>; 16] = [
        new_rgba(0xFF, 0x9D, 0x00),
        new_rgba(0x00, 0x00, 0xFF),
        new_rgba(0x20, 0xE2, 0xFF),
        new_rgba(0x00, 0xFF, 0x04),
        new_rgba(0xBC, 0x04, 0x6C),
        new_rgba(0x3C, 0x73, 0x00),
        new_rgba(0x05, 0x88, 0xFF),
        new_rgba(0x67, 0x2A, 0x00),
        new_rgba(0xFF, 0xC6, 0xF5),
        new_rgba(0xF3, 0xFF, 0x00),
        new_rgba(0xFF, 0xF7, 0xB3),
        new_rgba(0x00, 0xFF, 0xAF),
        new_rgba(0x00, 0x46, 0x4A),
        new_rgba(0xFF, 0x00, 0x29),
        new_rgba(0x15, 0x00, 0x47),
        new_rgba(0xFF, 0x00, 0xF5),
    ];

    const W: u32 = 256;
    const H: u32 = 240;
    const SRC: (i32, i32) = (127, 119);

    let mut img = RgbaImage::new(W, H);

    for dst in itertools::iproduct!(0..W as i32, 0..H as i32) {
        let i = aim(SRC, dst);
        img.put_pixel(dst.0 as u32, dst.1 as u32, COLORS[usize::from(i)]);
    }

    img
}

const fn new_rgba(r: u8, g: u8, b: u8) -> Rgba<u8> {
    Rgba([r, g, b, 0xFF])
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();

    let img = visualize();
    img.save(opt.path_out)?;

    Ok(())
}
