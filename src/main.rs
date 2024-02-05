use clap::{arg, command, value_parser};
use image::{GenericImage, GenericImageView, Pixel, Rgb};

fn hsl_sort(
    pixels: Vec<Rgb<u8>>,
    sort_func: fn(&Rgb<u8>) -> f32,
    exclude_func: fn(&Rgb<u8>) -> f32,
    lower: f32,
    upper: f32,
) -> Vec<Vec<Rgb<u8>>> {
    let mut chunks: Vec<Vec<Rgb<u8>>> = vec![];

    let mut group = vec![];
    for i in pixels {
        // could store this; computed twice
        let val = exclude_func(&i);
        if val < lower || val > upper {
            group.sort_by_key(|i| (sort_func(i) * 100.0) as u32);
            group.push(i);
            chunks.push(group.clone());
            group.clear();
        } else {
            group.push(i);
        }
    }

    chunks
}

fn luminance(pixel: &Rgb<u8>) -> f32 {
    (pixel.0.iter().max().unwrap().to_owned() as f32 / 255.0
        + pixel.0.iter().min().unwrap().to_owned() as f32 / 255.0)
        / 2.0
}

fn saturation(pixel: &Rgb<u8>) -> f32 {
    let min = pixel.0.iter().min().unwrap().to_owned() as f32;
    let max = pixel.0.iter().max().unwrap().to_owned() as f32;

    // no saturation
    if min == max {
        return 0.0;
    }
    // different formula if luminance > 50%
    if (min + max) / 2.0 > 0.5 {
        (max - min) / (max + min)
    } else {
        (max - min) / (2.0 - max - min)
    }
}

fn get_hsl_func(func_name: &str) -> fn(pixel: &Rgb<u8>) -> f32 {
    match func_name {
        "lightness" | "lightness_threshold" => luminance,
        "saturation" | "saturation_threshold" => saturation,
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
            .value_parser(["lightness_threshold", "saturation_threshold"])
            .default_value("lightness_threshold")
        )
        .arg(
            arg!(
                --lower_threshold [value] "Excludes pixels lower than this HSL value, e.g. excludes pixels darker than 10%"
            )
            .value_parser(value_parser!(f32))
            .default_value("0.25")
        )
        .arg(
            arg!(
                --upper_threshold [value] "Excludes pixels higher than this HSL value, e.g. excludes pixels more saturated than 60%"
            )
            .value_parser(value_parser!(f32))
            .default_value("0.8")
            .default_value_ifs([("sort", "saturation", Some("0.6"))])
        )
        .arg(
            arg!(
                -s --sort [value] "The pixel sorting algorithm to use"
            )
            .value_parser(["lightness", "saturation"])
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

        let grouped_cols = match matches.get_one::<String>("sort").unwrap().as_str() {
            "lightness" | "saturation" => hsl_sort(
                col,
                get_hsl_func(matches.get_one::<String>("sort").unwrap()),
                get_hsl_func(matches.get_one::<String>("exclude").unwrap()),
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
