mod img;
mod work;
mod ui;

#[cfg(test)]
mod test;

fn main() {
}

fn cli() {
    println!("Hello, world!");
    let mut args = std::env::args();
    let _executable = args.next().unwrap();
    let mut storedir = String::new();
    let mut filepath = String::new();
    let mut sim_thresh = 0.92f32;
    while args.len() > 0 {
        if let Some(a) = args.next() {
            match &a[..] {
                "--store" => {
                    storedir = args.next().expect("store directory bad");
                }
                "--sim" => {
                    sim_thresh = args
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
    work::process(&storedir, &filepath, sim_thresh);
}
