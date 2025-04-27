use std::path::Path;
use std::fs::File;
use serde::Deserialize;

#[derive(Deserialize)]
struct PostData {
    post_names: Vec<String>,
}

fn main() {
    println!("cargo::rerun-if-changed=../post_data.json");
    let path = Path::new("../post_data.json");
    let file = File::open(path)
        .expect("post_data.json file must exist");

    let post_data: PostData = serde_json::from_reader(file)
        .expect("post_data.json should be valid json");

    let post_names = post_data.post_names.iter()
        .map(|pn| format!("\"{pn}\"")).collect::<Vec<String>>();

    let data = format!("pub const POST_NAMES: [&'static str; {}] = [{}];", post_names.len(), post_names.join(","));
    std::fs::write("src/post_data.rs", data).expect("Unable to write to file");
}
