use std::path::PathBuf;

use structopt::StructOpt;

use grid_vis::GridVis;

use starsoldier_visualize::*;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    path_rom: PathBuf,

    #[structopt(parse(from_os_str))]
    path_out: PathBuf,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum GridCell {
    Empty,
    Origin,
    Dir(u8),
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<GridCell>>,
}

impl Grid {
    fn new(w: usize, h: usize, origin: (i32, i32), dirs: &[(i8, i8)]) -> Self {
        assert!(w > 0);
        assert!(h > 0);

        let mut cells = vec![vec![GridCell::Empty; w]; h];
        cells[origin.1 as usize][origin.0 as usize] = GridCell::Origin;

        let ps = dirs.iter().filter_map(|&(dx, dy)| {
            let x = origin.0 + i32::from(dx);
            let y = origin.1 + i32::from(dy);
            ((0..w as i32).contains(&x) && (0..h as i32).contains(&y))
                .then(|| (x as usize, y as usize))
        });

        for (i, (x, y)) in ps.enumerate() {
            cells[y][x] = GridCell::Dir(i as u8);
        }

        Self { cells }
    }
}

impl GridVis for Grid {
    fn col_count(&self) -> usize {
        self.cells[0].len()
    }
    fn row_count(&self) -> usize {
        self.cells.len()
    }

    fn line_color(&self) -> [u8; 3] {
        [0x00, 0x00, 0x00]
    }

    fn cell_width(&self) -> u32 {
        24
    }
    fn cell_height(&self) -> u32 {
        24
    }
    fn font_size(&self) -> u32 {
        24
    }

    fn cell_color(&self, x: usize, y: usize) -> [u8; 3] {
        match self.cells[y][x] {
            GridCell::Empty => [0xC0, 0xC0, 0xC0],
            GridCell::Origin => [0xFF, 0xFF, 0x00],
            GridCell::Dir(_) => [0xFF, 0xFF, 0xFF],
        }
    }

    fn cell_text(&self, x: usize, y: usize) -> Option<(String, [u8; 3])> {
        match self.cells[y][x] {
            GridCell::Empty => None,
            GridCell::Origin => Some(("弾".to_owned(), [0x00, 0x00, 0x00])),
            GridCell::Dir(idx) => {
                const COLORS: &[[u8; 3]] = &[
                    [0x00, 0x00, 0x00],
                    [0x00, 0x00, 0xFF],
                    [0x00, 0x80, 0x00],
                    [0xFF, 0x00, 0x00],
                ];
                let color = COLORS[usize::from(idx >> 4)];
                Some((format!("{:02X}", idx), color))
            }
        }
    }
}

fn visualize(dirs: &[(i8, i8)]) -> image::RgbaImage {
    let grid = Grid::new(21, 21, (10, 10), dirs);

    grid_vis::image::visualize(&grid)
}

fn get_directions(rom: &Rom) -> Vec<(i8, i8)> {
    let prg = &rom.prg;

    let dxs = prg[0x0923..][..0x40].iter().map(|&b| b as i8);
    let dys = prg[0x0963..][..0x40].iter().map(|&b| b as i8);

    dxs.zip(dys).collect()
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();
    let rom = Rom::from_ines(std::fs::read(opt.path_rom)?)?;

    let dirs = get_directions(&rom);

    let img = visualize(&dirs);
    img.save(opt.path_out)?;

    Ok(())
}
