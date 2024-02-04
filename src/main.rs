use clap::{arg, command, value_parser};
use image::{GenericImage, GenericImageView, Pixel, Rgb};


fn lightness_sort(pixels: Vec<Rgb<u8>>, lower: f32, upper: f32) -> Vec<Vec<Rgb<u8>>> {
    let mut chunks: Vec<Vec<Rgb<u8>>> = vec![];

    let mut group = vec![];
    for i in pixels {
        let lvl = lightness(&i) as f32;
        if lvl < lower * 255.0 || lvl > upper * 255.0 {
            group.sort_by_key(lightness);
            group.push(i);
            chunks.push(group.clone());
            group.clear();
        } else {
            group.push(i);
        }
    }

    chunks
}

fn lightness(pixel: &Rgb<u8>) -> u16 {
    (pixel.0.iter().max().unwrap().to_owned() as u16 + pixel.0.iter().min().unwrap().to_owned() as u16) / 2
    // pixel.0[0] as u16 + pixel.0[1] as u16 + pixel.0[2] as u16
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
            .value_parser(["lightness_threshold"])
            .default_value("lightness_threshold")
            .requires_if("lightness_threshold", "lightness_upper_threshold")
            .requires_if("lightness_threshold", "lightness_lower_threshold")
        )
        .arg(
            arg!(
                --lightness_lower_threshold [value] "Excludes pixels darker than this value"
            )
            .value_parser(value_parser!(f32))
            .default_value("0.25")
        )
        .arg(
            arg!(
                --lightness_upper_threshold [value] "Excludes pixels brighter than this value"
            )
            .value_parser(value_parser!(f32))
            .default_value("0.8")
        )
        .arg(
            arg!(
                -s --sort [value] "The pixel sorting algorithm to use"
            )
            .value_parser(["lightness"])
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
        _ => source
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
            "lightness" => lightness_sort(
                col,
                *matches.get_one("lightness_lower_threshold").unwrap(),
                *matches.get_one("lightness_upper_threshold").unwrap()
            ),
            _ => panic!("🤠")
        };
        for (c, i) in grouped_cols.concat().iter().enumerate() {
            target.put_pixel(x, c as u32, i.to_rgba())
        }
    }

    target = match matches.get_one::<String>("rotate").unwrap().as_str() {
        "0" => target,
        "90" =>  target.rotate270(),
        "180" => target.rotate180(),
        "270" => target.rotate90(),
        _ => target
    };
    target.save(matches.get_one::<String>("output").unwrap()).expect("Something went wrong with saving the file...");

    Ok(())
}
