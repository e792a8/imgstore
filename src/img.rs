use std::{fs::metadata, path::Path};

use image::{io::Reader as ImageReader, DynamicImage, ImageResult};
use image_hasher::{HasherConfig, ImageHash};

type Data = ImageHash;

pub struct ImgData {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub res: (u32, u32),
    pub data: Data,
}

pub fn get_img_data(path: &str) -> Result<ImgData, String> {
    match (metadata(path), open_img(path)) {
        (Ok(md), Ok(img)) => Ok(ImgData {
            path: path.to_owned(),
            name: Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            res: (img.width(), img.height()),
            size: md.len(),
            data: to_data(img),
        }),
        e => {
            if let Err(x) = e.0 {
                Err(x.to_string())
            } else {
                Err(e.1.err().unwrap().to_string())
            }
        }
    }
}

fn open_img(a: &str) -> ImageResult<DynamicImage> {
    Ok(ImageReader::open(a)?.with_guessed_format()?.decode()?)
}

fn to_data(img: DynamicImage) -> Data {
    let hasher = HasherConfig::new().to_hasher();
    hasher.hash_image(&img)
}

pub fn similarity(a: &Data, b: &Data) -> f32 {
    let rdist = a.dist(b) as f32 / (a.as_bytes().len() * 8) as f32;
    return 1f32 - rdist;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    pub fn similarity_check() {
        let mut args = std::env::args();
        let f1 = args.nth_back(0).unwrap();
        let f2 = args.nth_back(0).unwrap();
        let f1 = to_data(open_img(&f1).unwrap());
        let f2 = to_data(open_img(&f2).unwrap());
        let sim = similarity(&f1, &f2);
        println!("{} {} {sim}", f1.as_bytes().len(), f2.as_bytes().len());
    }
}
