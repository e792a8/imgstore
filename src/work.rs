use crate::img::{get_img_info_and_hist, get_img_info_and_sim, ImgInfo, ImgInfoAndSim};

use std::{
    fs::{metadata, read_dir},
    path::Path,
};

pub fn process(storedir: &str, filepath: &str, sim_th: f32) {
    match get_img_info_and_hist(filepath) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok((finfo, fhist)) => {
            let mut dv = Vec::<ImgInfoAndSim>::new();
            let storepath = Path::new(&storedir[..]);
            let dir = read_dir(&storepath).expect("load store directory failed");
            let total = dir.count() as u32;
            let dir = read_dir(&storepath).expect("load store directory failed");
            let mut cnt = 0;
            progress(0, total, &wnd);
            status("loading", &wnd);
            for elem in dir {
                let elem = storepath
                    .join(elem.unwrap().path())
                    .to_str()
                    .unwrap()
                    .to_owned();
                if metadata(&elem).unwrap().is_file() {
                    match get_img_info_and_sim(&fhist, &elem) {
                        Err(e) => message(&format!("{}: {}", elem, e), &wnd),
                        Ok(info) => {
                            if Path::new(&finfo.path).file_name() == Path::new(&elem).file_name()
                                || info.1 > sim_th
                            {
                                dv.push(info);
                            }
                        }
                    }
                }
                cnt += 1;
                progress(cnt, total, &wnd);
            }
            let storedir = storedir.to_string();
            status("main", &wnd);
            ask(&finfo, &dv, &wnd);
            answer(finfo, dv, storedir, wnd);
        }
    }
}

fn message(a: &str, wnd: &Window) {
    wnd.emit("message", a).unwrap();
    println!("{}", a);
}

fn progress(done: u32, total: u32, wnd: &tauri::Window) {
    wnd.emit("progress", (done, total)).unwrap();
}

type ImgsPayload<'a> = (&'a ImgInfo, &'a Vec<ImgInfoAndSim>);

fn status(a: &str, wnd: &Window) {
    wnd.emit("status", a).unwrap();
}

fn ask(a: &ImgInfo, b: &Vec<ImgInfoAndSim>, wnd: &tauri::Window) {
    wnd.emit("imgs", (a, b) as ImgsPayload).unwrap();
}

#[derive(Serialize, Deserialize)]
struct Answer(String, Vec<String>);

pub fn answer(a: ImgInfo, b: Vec<ImgInfoAndSim>, storedir: String, w: tauri::Window) {
    let wnd = w.clone();
    w.once("answer", move |ev| {
        let ans = ev.payload().unwrap();
        let ans: Answer = serde_json::from_str(ans).unwrap();
        for i in 0..b.len() {
            match &ans.1[i][..] {
                "K" => {
                    message(&format!("{}: Keep", b[i].0.path), &wnd);
                }
                "D" => {
                    message(&format!("{}: Delete", b[i].0.path), &wnd);
                    std::fs::remove_file(&b[i].0.path).unwrap();
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
        status("done", &wnd);
    });
}
