mod exclude;
mod sort;
mod cli;

use std::error::Error;

use image::{GenericImage, GenericImageView, Pixel, Rgb};

use crate::exclude::{hsl_exclude, random_exclude};
use crate::sort::{hue, saturation, luminance};
use crate::cli::Cli;


fn get_hsl_func(func_name: &str) -> fn(pixel: &Rgb<u8>) -> f32 {
    match func_name {
        "lightness" | "lightness_threshold" => luminance,
        "saturation" | "saturation_threshold" => saturation,
        "hue" | "hue_threshold" => hue,
        _ => panic!("Unknown HSL function name: {}", func_name),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();

    let mut source = image::open(cli.input_file).unwrap();
    source = match cli.rotate {
        0 => source,
        90 => source.rotate90(),
        180 => source.rotate180(),
        270 => source.rotate270(),
        _ => source,
    };
    let dims = source.dimensions();
    let mut target = source.clone();

    for x in 0..dims.0 {
        let mut col = vec![];
        for y in 0..dims.1 {
            let pixel = source.get_pixel(x, y).to_rgb();
            col.push(pixel);
        }

        let grouped_cols = match cli.exclude_algorithm.as_str() {
            "lightness_threshold" | "saturation_threshold" | "hue_threshold" => hsl_exclude(
                col,
                get_hsl_func(cli.sort_algorithm.as_str()),
                get_hsl_func(cli.exclude_algorithm.as_str()),
                cli.lower_threshold,
                cli.upper_threshold,
            ),
            "random_exclude" => random_exclude(
                col,
                get_hsl_func(cli.sort_algorithm.as_str()),
                cli.lower_threshold,
                cli.upper_threshold,
            ),
            _ => panic!("Unknown pixel exclusion algorithm"),
        };
        for (c, i) in grouped_cols.concat().iter().enumerate() {
            target.put_pixel(x, c as u32, i.to_rgba())
        }
    }

    target = match cli.rotate {
        0 => target,
        90 => target.rotate270(),
        180 => target.rotate180(),
        270 => target.rotate90(),
        _ => target,
    };
    target
        .save(cli.output_file)
        .expect("Something went wrong with saving the file...");

    Ok(())
}
