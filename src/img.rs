use std::{fs::metadata, path::Path};

use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, ImageResult};
pub struct ImgInfo {
    pub path: String,
    pub filename: String,
    pub resolution: (u32, u32),
    pub size: u64,
}

pub struct ImgInfoAndSim(pub ImgInfo, pub f32);
pub type NormalizedHist = [[f32; 64]; 3];

pub fn get_img_info_and_hist(path: &str) -> Result<(ImgInfo, NormalizedHist), String> {
    match (metadata(path), open_img(path)) {
        (Ok(md), Ok(img)) => Ok((
            ImgInfo {
                path: path.to_owned(),
                filename: Path::new(path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                resolution: (img.width(), img.height()),
                size: md.len(),
            },
            normalized_hist(&img),
        )),
        e => {
            if let Err(_) = e.0 {
                Err(e.0.err().unwrap().to_string())
            } else {
                Err(e.1.err().unwrap().to_string())
            }
        }
    }
}

pub fn get_img_info_and_sim(h: &NormalizedHist, path: &str) -> Result<ImgInfoAndSim, String> {
    match (metadata(path), open_img(path)) {
        (Ok(md), Ok(img)) => Ok(ImgInfoAndSim(
            ImgInfo {
                path: path.to_owned(),
                filename: Path::new(path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                resolution: (img.width(), img.height()),
                size: md.len(),
            },
            hist_similarity(h, &normalized_hist(&img)),
        )),
        e => {
            if let Err(_) = e.0 {
                Err(e.0.err().unwrap().to_string())
            } else {
                Err(e.1.err().unwrap().to_string())
            }
        }
    }
}

pub fn open_img(a: &str) -> ImageResult<DynamicImage> {
    Ok(ImageReader::open(a)?.with_guessed_format()?.decode()?)
}

pub fn normalized_hist(img: &DynamicImage) -> NormalizedHist {
    let mut hist = [[0u32; 256]; 3];
    for p in img.pixels() {
        hist[0][p.2 .0[0] as usize] += 1;
        hist[1][p.2 .0[1] as usize] += 1;
        hist[2][p.2 .0[2] as usize] += 1;
    }
    let mut norm = [[0f32; 64]; 3];
    for c in 0..3 {
        let mut m = 1u32;
        for i in 0..256 {
            m = m.max(hist[c][i]);
        }
        for i in 0..256 {
            norm[c][i / 4] += hist[c][i] as f32 / m as f32 / 4f32;
        }
    }
    norm
}

pub fn hist_similarity(a: &[[f32; 64]; 3], b: &[[f32; 64]; 3]) -> f32 {
    let mut diff = 0f32;
    for c in 0..3 {
        for i in 0..64 {
            diff += (a[c][i] - b[c][i]).abs()
        }
    }
    1f32 - diff / 3f32 / 64f32
}

pub fn img_similarity(a: &DynamicImage, b: &DynamicImage) -> f32 {
    let a = normalized_hist(&a);
    let b = normalized_hist(&b);
    hist_similarity(&a, &b)
}
