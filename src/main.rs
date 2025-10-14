use image::{GrayImage, ImageBuffer, ImageReader, Luma};
use std::path::{PathBuf, absolute};

fn main() {
    make_straight();
}
pub fn make_straight() {
    let root = "C:/users/kloz1/desktop/alphabet/";
    let pathbuf = PathBuf::from(root).join("B.png");

    // let image_bytes = fs::read(pathbuf).expect("Failed to read texture file.");

    let dy_image = ImageReader::open(pathbuf)
        .unwrap()
        .decode()
        .expect("failed to open image.");

    let image = dy_image.into_luma8();

    let factor = 16;

    let (width, height) = image.dimensions();
    let image_data = image.into_raw();
    let image_size = image_data.len();
    let mut grey_pixel_locations: Vec<(f64, f64)> = Vec::new();
    let mut white_pixel_locations: Vec<(f64, f64)> = Vec::new();
    let mut pos = 1;
    let mut row = 0.0;
    let mut column = 0.0;

    for byte in image_data {
        if byte > 171 && byte < 190 {
            let x = column - 0.5;
            let y = row - 0.5;
            grey_pixel_locations.push((x, y));
        } else {
            let x = column - 0.5;
            let y = row - 0.5;
            white_pixel_locations.push((x, y));
        }

        if pos % width == 0 {
            row += 1.0;
        }
        if column % width as f64 == 0.0 {
            column = 0.0;
        }
        column += 1.0;
        pos += 1;
    }

    let mut distance_field: Vec<Vec<f64>> = vec![vec![0.0; factor]; factor];
    let mut distance_field_pos: Vec<Vec<(f64, f64)>> = vec![vec![(0.0, 0.0); factor]; factor];

    for y in 0..factor {
        for x in 0..factor {
            distance_field_pos[x][y] = (
                x as f64 * (width as f64 / factor as f64) + 0.5,
                y as f64 * (width as f64 / factor as f64) + 0.5,
            );
        }
    }

    for y in 0..factor {
        for x in 0..factor {
            let mut in_grey: bool = false;
            for tuple in &grey_pixel_locations {
                if tuple.0 == distance_field_pos[y][x].0 && tuple.1 == distance_field_pos[y][x].1 {
                    in_grey = true;
                }
            }

            let mut closest_distance = -1000000.0;
            if in_grey == true {
                closest_distance = 100000.0;
                for tupler in &white_pixel_locations {
                    let s1 = tupler.0 - distance_field_pos[y][x].0;
                    let s2 = tupler.1 - distance_field_pos[y][x].1;
                    let distance = ((s1 * s1) + (s2 * s2)).sqrt();
                    if distance < closest_distance {
                        closest_distance = distance;
                    }
                }
            } else {
                for tuple in &grey_pixel_locations {
                    let s1 = tuple.0 - distance_field_pos[y][x].0;
                    let s2 = tuple.1 - distance_field_pos[y][x].1;
                    let distance = -((s1 * s1) + (s2 * s2)).sqrt();
                    if distance > closest_distance {
                        closest_distance = distance;
                    }
                }
            }
            distance_field[x][y] = closest_distance;
        }
    }

    let mut least = 0.0;
    let mut most = 0.0;

    for y in 0..factor {
        for x in 0..factor {
            if distance_field[x][y] > most {
                most = distance_field[x][y];
            } else if distance_field[x][y] < least {
                least = distance_field[x][y];
            }
        }
    }
    let range = (least.abs() + most) as f64;
    let mapp = range / 255.0;

    let mut bytes: Vec<u8> = Vec::new();
    for y in 0..factor {
        for x in 0..factor {
            distance_field[y][x] += least.abs();
            bytes.push((distance_field[y][x] / mapp) as u8);
        }
    }

    // for byte in bytes {
    //     println!("byte {}", byte);
    // }

    println!("most! {}, least! {} map! {}", most, least, mapp);
    println!(
        "df pos {},{} df value {}",
        distance_field_pos[0][0].0, distance_field_pos[0][0].1, distance_field[0][0]
    );

    let mut image: GrayImage = ImageBuffer::new(factor as u32, factor as u32);

    let mut i = 0;
    for y in 0..factor as u32 {
        for x in 0..factor as u32 {
            image.put_pixel(x, y, Luma([bytes[i]]));
            i += 1;
        }
    }
    let pathbuf = PathBuf::from(root).join("B+.png");
    image.save(pathbuf).unwrap();
}

pub fn make_wavy() {
    let root = "C:/users/kloz1/desktop/alphabet/";
    let pathbuf = PathBuf::from(root).join("B.png");

    // let image_bytes = fs::read(pathbuf).expect("Failed to read texture file.");

    let dy_image = ImageReader::open(pathbuf)
        .unwrap()
        .decode()
        .expect("failed to open image.");

    let image = dy_image.into_luma8();

    let (width, height) = image.dimensions();
    let image_data = image.into_raw();
    let image_size = image_data.len();

    let mut pos: u32 = 0;
    let mut row: usize = 0;
    let factor = 16;
    let mut new_data: Vec<u32> = vec![0; 16 * 16];
    let mut new_data_column = 0;
    let mut new_data_row = 0;

    for byte in image_data {
        if byte > 171 && byte < 190 {
            let index = (new_data_column + (new_data_row * factor)) - 1;
            if index == 0 {
                println!("yelp");
            }
            if index < 256 {
                new_data[index as usize] += 1;
            }
        }

        pos += 1;

        if pos % width == 0 {
            row += 1;
            new_data_column = 0;
            if row % (width / factor) as usize == 0 {
                new_data_row += 1;
            }
        }
        if pos % (width / factor) == 0 {
            new_data_column += 1;
        }
    }

    println!("size: {}, width: {}, height: {}", image_size, width, height);
    println!("rows: {}", new_data_column);
    let mut new_new_data: Vec<u8> = Vec::new();
    for data in new_data {
        let new_haha: u8 = (data / (65536 / 255)) as u8;
        new_new_data.push(new_haha);
    }

    let mut image: GrayImage = ImageBuffer::new(factor, factor);
    let mut i = 0;
    for y in 0..factor {
        for x in 0..factor {
            image.put_pixel(x, y, Luma([new_new_data[i]]));
            i += 1;
        }
    }

    let pathbuf = PathBuf::from(root).join("B+.png");
    image.save(pathbuf).unwrap();
}
