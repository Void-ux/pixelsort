use image::Rgb;

pub fn luminance(pixel: &Rgb<u8>) -> f32 {
    (pixel.0.iter().max().unwrap().to_owned() as f32 / 255.0
        + pixel.0.iter().min().unwrap().to_owned() as f32 / 255.0)
        / 2.0
}

pub fn saturation(pixel: &Rgb<u8>) -> f32 {
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

pub fn hue(pixel: &Rgb<u8>) -> f32 {
    let min = pixel.0.iter().min().unwrap().to_owned() as i32;
    let max = pixel.0.iter().max().unwrap().to_owned() as i32;
    if max - min == 0 {
        return 0.0;
    }

    let r = pixel.0[0] as i32;
    let g = pixel.0[1] as i32;
    let b = pixel.0[2] as i32;

    let mut _hue: i32;
    if r == max {
        _hue = g - b / (max - min);
    } else if g == max {
        _hue = 2 + (b - r) / (max - min);
    } else {
        _hue = 4 + (b - g) / (max - min);
    }

    _hue *= 60;
    if _hue < 0 {
        _hue += 360;
    }
    (_hue / 360) as f32
}
