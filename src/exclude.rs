use rand::Rng;
use image::Rgb;

pub fn random_exclude(pixels: Vec<Rgb<u8>>, sort_func: fn(&Rgb<u8>) -> f32, lower: f32, upper: f32) -> Vec<Vec<Rgb<u8>>> {
    let mut chunks: Vec<Vec<Rgb<u8>>> = vec![];

    let mut group = vec![];
    for i in pixels {
        // could store this; computed twice
        let num = rand::thread_rng().gen_range(lower as usize..upper as usize);
        if num == 0 {
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

pub fn hsl_exclude(
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
