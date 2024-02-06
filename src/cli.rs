use clap::{arg, builder::{ArgPredicate, OsStr}, value_parser, ArgMatches};

pub struct Cli {
    #[allow(dead_code)]
    matches: ArgMatches,
    pub input_file: String,
    pub output_file: String,
    pub exclude_algorithm: String,
    pub lower_threshold: f32,
    pub upper_threshold: f32,
    pub sort_algorithm: String,
    pub rotate: u16
}

impl Cli {
    pub fn from_args() -> Self {
        let matches = clap::Command::new("pixelsort")
            .about("Add unique, glitchy effects to your images by sorting pixels")
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

        let input_file = matches.get_one::<String>("name").unwrap().to_owned();
        let output_file = matches.get_one::<String>("output").unwrap().to_owned();
        let exclude_algorithm = matches.get_one::<String>("exclude").unwrap().to_owned();
        let lower_threshold = *matches.get_one::<f32>("lower_threshold").unwrap();
        let upper_threshold = *matches.get_one::<f32>("upper_threshold").unwrap();
        let sort_algorithm = matches.get_one::<String>("sort").unwrap().to_owned();
        let rotate = matches.get_one::<String>("rotate").unwrap().parse::<u16>().unwrap();

        Cli {
            matches,
            input_file,
            output_file,
            exclude_algorithm,
            lower_threshold,
            upper_threshold,
            sort_algorithm,
            rotate
        }
    }
}
