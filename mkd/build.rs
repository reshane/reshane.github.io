use std::path::Path;
use std::fs::File;
use serde::Deserialize;

#[derive(Deserialize)]
struct PostData {
    post_names: Vec<String>,
    demo_names: Vec<String>,
}

fn main() {
    println!("cargo::rerun-if-changed=../site_data.json");
    let path = Path::new("../site_data.json");
    let file = File::open(path)
        .expect("post_data.json file must exist");

    let data: PostData = serde_json::from_reader(file)
        .expect("post_data.json should be valid json");

    let post_names = data.post_names.iter()
        .map(|pn| format!("\"{pn}\"")).collect::<Vec<String>>();
    let demo_names = data.demo_names.iter()
        .map(|dn| format!("\"{dn}\"")).collect::<Vec<String>>();

    let post_data = format!("pub const POST_NAMES: [&'static str; {}] = [{}];", post_names.len(), post_names.join(","));
    let demo_data = format!("pub const DEMO_NAMES: [&'static str; {}] = [{}];", demo_names.len(), demo_names.join(","));
    let site_data = vec![post_data, demo_data].join("\n");
    std::fs::write("src/site_data.rs", site_data).expect("Unable to write to file");
}
