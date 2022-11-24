use std::{
    path::Path,
    fs::File,
    io::Write
};
use rand::random;

const SEEDS_COUNT: usize = 10;
const HEIGHT: usize = 600;
const WIDTH: usize = 800;

const RED: u32 = 0xFF0000FF;
const WHITE: u32 = 0xFFFFFFFF;

const BRIGHT_RED: u32 = 0xFFfb4934;
const BRIGHT_GREEN: u32 = 0xFFb8bb26;
const BRIGHT_YELLOW: u32 = 0xFFfabd2f;
const BRIGHT_BLUE: u32 = 0xFF83a598;
const BRIGHT_PURPLE: u32 = 0xFFd3869b;
const BRIGHT_AQUA: u32 = 0xFF8ec07c;
const BRIGHT_ORANGE: u32 = 0xFFfe8019;

pub struct Voronoi {
    points: [Point;SEEDS_COUNT],
    image: Vec<Vec<u32>>
}

#[derive(Copy, Clone)]
struct Point(usize, usize); 
    
impl Voronoi {
    pub fn new() -> Self {
        let mut points: [Point;SEEDS_COUNT] = [Point(0,0);SEEDS_COUNT];

        for i in 0..SEEDS_COUNT {
            points[i].0 = random::<usize>()%WIDTH;
            points[i].1 = random::<usize>()%HEIGHT;
        }

        Self {
            points,
            image: vec![vec![BRIGHT_AQUA; WIDTH]; HEIGHT]
        }
    }

    pub fn save_image<T: AsRef<Path> + std::fmt::Display>(&self, path: T) {
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => panic!("Couldn't create file: [{}] because: [{}]", path, err)
        };

        let mut image: String = format!("P3\n{WIDTH} {HEIGHT} 255\n");

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // 0xAA(Alpha)BB(Blue)GG(Green)RR(Red)
                
                let pixel: u32 = self.image[y][x];

                let bytes: [u32;3] = [
                    (pixel & 0x0000FF),
                    (pixel & 0x00FF00) >> 8,
                    (pixel & 0xFF0000) >> 16,
                ];

                image.push_str(&format!("{} {} {} ", bytes[0], bytes[1], bytes[2]));
            }
            image.pop();
            image.push('\n');
        }
        write!(file, "{}", &image).unwrap();
    }
}
