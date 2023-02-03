mod img;
mod work;

#[cfg(test)]
mod test;

fn main() {
    use work::*;
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_cmd_args, process, answer,])
        .build(tauri::generate_context!())
        .expect("tauri app error");
    app.run(|_, _| {});
}
