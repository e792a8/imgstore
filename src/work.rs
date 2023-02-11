use crate::img::{get_img_data, hist_similarity, ImgData};

use serde::{Deserialize, Serialize};
use std::{
    fs::{metadata, read_dir},
    path::Path,
};
use tauri::Window;

#[derive(Serialize, Deserialize)]
pub struct Args {
    storedir: String,
    filepath: String,
    sim_th: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MainImgInfo {
    path: String,
    name: String,
    size: u64,
    res: (u32, u32),
}

#[derive(Serialize, Deserialize)]
pub struct ImgElemInfo {
    path: String,
    name: String,
    size: u64,
    res: (u32, u32),
    sim: f32,
}

type ImgsInfo = (MainImgInfo, Vec<ImgElemInfo>);

type Answer = (String, Vec<String>);

#[tauri::command(async)]
pub fn get_cmd_args() -> Args {
    let mut args = std::env::args();
    let _executable = args.next().unwrap();
    let mut storedir = String::new();
    let mut filepath = String::new();
    let mut sim_th = 0.92f32;
    while args.len() > 0 {
        if let Some(a) = args.next() {
            match &a[..] {
                "--store" => {
                    storedir = args.next().expect("store directory bad");
                }
                "--sim" => {
                    sim_th = args
                        .next()
                        .unwrap()
                        .parse()
                        .expect("similarity threshold bad");
                }
                _ => {
                    filepath = a;
                }
            }
        }
    }
    assert!(storedir.len() > 0, "store directory not specified");
    assert!(filepath.len() > 0, "file path not specified");
    let storedir = Path::new(&storedir)
        .canonicalize()
        .expect("store directory bad")
        .to_str()
        .unwrap()
        .to_string();
    let filepath = Path::new(&filepath)
        .canonicalize()
        .expect("file path bad")
        .to_str()
        .unwrap()
        .to_string();
    Args {
        storedir,
        filepath,
        sim_th,
    }
}

#[tauri::command(async)]
pub fn process(args: Args, wnd: tauri::Window) -> Option<ImgsInfo> {
    let storedir = &args.storedir;
    let filepath = &args.filepath;
    let sim_th = &args.sim_th;
    match get_img_data(filepath) {
        Err(e) => {
            message(&format!("{}", e), &wnd);
            return None;
        }
        Ok(fdata) => {
            let mut elems = Vec::<ImgElemInfo>::new();
            let storepath = Path::new(&storedir[..]);
            let dir = read_dir(&storepath).expect("load store directory failed");
            let total = dir.count() as u32;
            let dir = read_dir(&storepath).expect("load store directory failed");
            let mut cnt = 0;
            progress(0, total, &wnd);
            for elem in dir {
                let elem = storepath
                    .join(elem.unwrap().path())
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned();
                if metadata(&elem).unwrap().is_file() {
                    match get_img_data(&elem) {
                        Err(e) => message(&format!("{}: {}", elem, e), &wnd),
                        Ok(ImgData {
                            path,
                            name,
                            size,
                            res,
                            hist,
                        }) => {
                            let sim = hist_similarity(&fdata.hist, &hist);
                            if Path::new(&fdata.path).file_name() == Path::new(&path).file_name()
                                || sim > *sim_th
                            {
                                let elem = ImgElemInfo {
                                    sim,
                                    path,
                                    name,
                                    size,
                                    res,
                                };
                                elems.push(elem);
                            }
                        }
                    }
                }
                cnt += 1;
                progress(cnt, total, &wnd);
            }
            let main_img = MainImgInfo {
                path: fdata.path,
                name: fdata.name,
                size: fdata.size,
                res: fdata.res,
            };
            Some((main_img, elems))
        }
    }
}

#[tauri::command(async)]
pub fn answer(args: Args, imgs: ImgsInfo, ans: Answer, wnd: tauri::Window) {
    let storedir = &args.storedir;
    let a = imgs.0;
    let b = imgs.1;
    assert_eq!(b.len(), ans.1.len());
    for i in 0..b.len() {
        match &ans.1[i][..] {
            "K" => {
                message(&format!("{}: Keep", b[i].path), &wnd);
            }
            "D" => {
                message(&format!("{}: Delete", b[i].path), &wnd);
                std::fs::remove_file(&b[i].path).unwrap();
            }
            _ => message(&format!("Unknown action [{}]: {}", i + 1, ans.1[i]), &wnd),
        }
    }
    let mut main_fname: String = Path::new(&a.path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .into();
    let main_fname_o = main_fname.clone();
    let mut suf = 1;
    while let Some(_) = read_dir(&storedir)
        .unwrap()
        .find(|e| e.as_ref().unwrap().file_name().to_str().unwrap() == main_fname)
    {
        main_fname = format!(
            "{}_{}.{}",
            Path::new(&main_fname_o)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
            suf,
            Path::new(&main_fname_o)
                .extension()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        suf += 1;
    }
    match &ans.0[..] {
        "K" => {
            message(&format!("{}: Keep", a.path), &wnd);
        }
        "D" => {
            message(&format!("{}: Delete", a.path), &wnd);
            std::fs::remove_file(a.path).unwrap()
        }
        "C" => {
            message(&format!("{}: Copy as: {}", a.path, main_fname), &wnd);
            std::fs::copy(a.path, Path::new(&storedir).join(&main_fname)).unwrap();
        }
        "M" => {
            message(&format!("{}: Move as: {}", a.path, main_fname), &wnd);
            std::fs::rename(a.path, Path::new(&storedir).join(&main_fname)).unwrap();
        }
        _ => message(&format!("Unknown action [M]: {}", ans.0), &wnd),
    }
}

fn message(a: &str, wnd: &Window) {
    wnd.emit("message", a).unwrap();
    println!("{}", a);
}

fn progress(done: u32, total: u32, wnd: &tauri::Window) {
    wnd.emit("progress", (done, total)).unwrap();
}
