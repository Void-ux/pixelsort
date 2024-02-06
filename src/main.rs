mod exclude;
mod sort;

use clap::{arg, builder::{ArgPredicate, OsStr}, command, value_parser};
use image::{GenericImage, GenericImageView, Pixel, Rgb};
use crate::exclude::{hsl_exclude, random_exclude};
use crate::sort::{hue, saturation, luminance};


fn get_hsl_func(func_name: &str) -> fn(pixel: &Rgb<u8>) -> f32 {
    match func_name {
        "lightness" | "lightness_threshold" => luminance,
        "saturation" | "saturation_threshold" => saturation,
        "hue" | "hue_threshold" => hue,
        _ => panic!("Unknown HSL function name: {}", func_name),
    }
}

fn main() -> Result<(), ()> {
    let matches = command!()
        .arg(arg!(<name> "The file path of the image to pixel sort"))
        .arg(
            arg!(
                -o --output [name] "The file path to output to"
            )
            .default_value("output.png")
        )
        .arg(
            arg!(
                -e --exclude [value] "Determines which pixels to exclude from sorting"
            )
            .value_parser(["lightness_threshold", "saturation_threshold", "hue_threshold", "random_exclude"])
            .default_value("lightness_threshold")
        )
        .arg(
            arg!(
                --lower_threshold [value] "Excludes pixels lower than this HSL value, e.g. excludes pixels darker than 10%"
            )
            .value_parser(value_parser!(f32))
            .default_value("0.25")
            .default_value_if("exclude", ArgPredicate::Equals(OsStr::from("random_exclude")), Some("0"))
        )
        .arg(
            arg!(
                --upper_threshold [value] "Excludes pixels higher than this HSL value, e.g. excludes pixels more saturated than 60%"
            )
            .value_parser(value_parser!(f32))
            .default_value("0.8")
            .default_value_ifs([
                ("sort", "saturation", Some("0.6")),
                ("exclude", "random_exclude", Some("5"))
            ])
        )
        .arg(
            arg!(
                -s --sort [value] "The pixel sorting algorithm to use"
            )
            .value_parser(["lightness", "saturation", "hue"])
            .default_value("lightness")
        )
        .arg(
            arg!(
                -r --rotate [value] "Amount to rotate the image by before processing"
            )
            .value_parser(["0", "90", "180", "270"])
            .default_value("0")
        )
        .get_matches();

    let mut source = image::open(matches.get_one::<String>("name").unwrap()).unwrap();
    source = match matches.get_one::<String>("rotate").unwrap().as_str() {
        "0" => source,
        "90" => source.rotate90(),
        "180" => source.rotate180(),
        "270" => source.rotate270(),
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

        let grouped_cols = match matches.get_one::<String>("exclude").unwrap().as_str() {
            "lightness_threshold" | "saturation_threshold" | "hue_threshold" => hsl_exclude(
                col,
                get_hsl_func(matches.get_one::<String>("sort").unwrap()),
                get_hsl_func(matches.get_one::<String>("exclude").unwrap()),
                *matches.get_one("lower_threshold").unwrap(),
                *matches.get_one("upper_threshold").unwrap(),
            ),
            "random_exclude" => random_exclude(
                col,
                get_hsl_func(matches.get_one::<String>("sort").unwrap()),
                *matches.get_one("lower_threshold").unwrap(),
                *matches.get_one("upper_threshold").unwrap(),
            ),
            _ => panic!("🤠"),
        };
        for (c, i) in grouped_cols.concat().iter().enumerate() {
            target.put_pixel(x, c as u32, i.to_rgba())
        }
    }

    target = match matches.get_one::<String>("rotate").unwrap().as_str() {
        "0" => target,
        "90" => target.rotate270(),
        "180" => target.rotate180(),
        "270" => target.rotate90(),
        _ => target,
    };
    target
        .save(matches.get_one::<String>("output").unwrap())
        .expect("Something went wrong with saving the file...");

    Ok(())
}
