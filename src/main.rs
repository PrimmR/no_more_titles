use std::io;
use image::{ImageBuffer, RgbImage, Rgb};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use itertools::Itertools;

const HEIGHT: i32 = 1080;
const WIDTH: i32 = HEIGHT * 16/9;

// Percentages
const TITLE_SIZE: f32 = 6.75;
const CAPTION_SIZE: f32 =  10.8;
const SPACING_FROM_MID: f32 = 10.8;
const MARGIN: f32 = 0.8;
const LINE_BREAK: f32 = 0.0125;
const MAX_LINES: i32 = 4;

const WIDE_TEXT: bool = true;

fn center(constraint: i32, size: i32) -> i32 {
    (constraint - size) / 2
}

fn draw_title(img: &mut RgbImage, font: &Font<'_>) {
    let text = "N O  M O R E";
    let factor = HEIGHT as f32 / TITLE_SIZE;
    let scale = Scale{x: factor, y: factor};
    let size = text_size(scale, font, text);
    let x = center(i32::try_from(WIDTH).unwrap(), size.0);
    let y = center(i32::try_from(HEIGHT).unwrap(), size.1);
    let y = y - (HEIGHT as f32 / SPACING_FROM_MID) as i32;

    draw_text_mut(img, Rgb([255, 255, 255]), x, y, scale, font, text);
}

fn draw_text(img: &mut RgbImage, font: &Font<'_>, text: Vec<String>) {
    let mut i = 0;
    for line in &text {
        let factor = HEIGHT as f32 / CAPTION_SIZE;
        let scale = Scale{x: factor, y: factor};
        let size = text_size(scale, font, line);
        let x = center(WIDTH, size.0);
        let y = center(HEIGHT, size.1);
        let y = y + (HEIGHT as f32 / SPACING_FROM_MID) as i32 + (size.1 + (HEIGHT as f32 * LINE_BREAK) as i32) * i;

        draw_text_mut(img, Rgb([255, 255, 255]), x, y, scale, font, line);
        i += 1;
    }
}

// Divides and floors floats to integers, floors integers down by 1
fn div_floor(num: i32, den: i32) -> i32 {
    if num % den == 0 {
        (num / den) - 1
    } else {
        num / den
    }
}

// Auto line wrap
fn split_text(text: String, font: &Font<'_>) -> Result<Vec<String>, String> {
    const LIMIT: i32 = (WIDTH as f32 * MARGIN) as i32;

    if text == "" {
        return Err(String::from("Please input text"));
    }

    let (text, split) = if WIDE_TEXT {
        (text.chars().join(" "), "  ")
    } else {
        (text, " ")
    };

    let factor = HEIGHT as f32 / CAPTION_SIZE;
    let scale = Scale{x: factor, y: factor};
    let size = text_size(scale, font, &text);

    let num_lines = size.0 / LIMIT + 1;

    if num_lines > MAX_LINES {
        return Err(String::from("Input too long"));
    }

    let target = size.0 / num_lines;
    let words: Vec<_> = text.split(split).collect();

    let mut lines = vec![String::from(""); num_lines as usize];
    for i in 0..words.len() {
        if text_size(scale, font, words[i]).0 > LIMIT {
            return Err(String::from("Your input contains a word that is too long"));
        }
        let acc_width = text_size(scale, font, words[..i+1].join(" ").as_str()).0;
        let line = div_floor(acc_width, target);
        lines[line as usize] += &format!("{} ", words[i]);
    }

    Ok(lines)
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(u32::try_from(WIDTH).unwrap(), u32::try_from(HEIGHT).unwrap());
    let start = Rgb([0, 0, 0]);
    let end = Rgb([26, 25, 28]);

    image::imageops::vertical_gradient(&mut img, &start, &end);

    let font = Vec::from(include_bytes!("FMedium.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let text;
    loop {
        println!("Category:");
        // let cat = String::from("Shorties");
        // let cat = String::from("This is a very long string nooo its run over");
        // let cat = String::from("This is a very long string and now it's even longer and now it's even longer");

        let mut cat = String::new();
        io::stdin().read_line(&mut cat).expect("failed to readline");

        match split_text(cat.trim().to_ascii_uppercase(), &font) {
            Ok(t) => {
                text = t;
                break
            },
            Err(e) => println!("{}", e),
        }
    }

    draw_title(&mut img, &font);
    draw_text(&mut img, &font, text);

    img.save("output.png").unwrap();
    println!("Written to output.png")
}