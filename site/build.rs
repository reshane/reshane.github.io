// building the html for the posts with the markdown and image files

use std::{
    collections::HashMap,
    io::Write,
    fs::{
        self,
        File,
    },
};

fn main() {
    println!("cargo::rerun-if-changed=posts/");
    let paths = fs::read_dir("./posts").unwrap();

    let mut blog_strings = HashMap::<String, String>::new();
    for path in paths {
        let path = path.unwrap();
        let post_name = format!("{}", path.file_name().clone().to_str().unwrap());
        let mut post_string_lines = Vec::<String>::new();
        let mut image_names = HashMap::<String, String>::new();
        if path.file_type().unwrap().is_dir() {
            for p in fs::read_dir(path.path()).unwrap() {
                let p = p.unwrap();
                let file_name = format!("{}", p.file_name().to_str().unwrap());
                if file_name == "blog.md" {
                    let post_string = fs::read_to_string(p.path()).unwrap();
                    post_string_lines = post_string.split("\n")
                        .map(|s: &str| s.to_string())
                        .collect();
                }
                if file_name == "images" {
                    // also copy this whole directory over to the new one
                    let target_dir = format!("build/{post_name}/images");
                    std::fs::create_dir_all(target_dir.clone()).unwrap();
                    for img in fs::read_dir(p.path()).unwrap() {
                        let img = img.unwrap();
                        if img.file_type().unwrap().is_file() {
                            let img_name = format!("{}", img.file_name().to_str().unwrap());
                            let _ = std::fs::copy(format!("{}", img.path().display()), format!("{}/{}", target_dir.clone(), img_name.clone()));
                            let idx = img_name.find("_").expect("Image name must be prefixed with unique id");
                            let key = String::from(&img_name[0..idx]);
                            image_names.insert(key.clone(), img_name.clone());
                        }
                    }
                }
            }
        }
        
        let mut post_string = String::new();
        let mut code_block = false;
        for mut line in post_string_lines {
            // replace all the image references properly
            if let (Some(s_idx), Some(e_idx)) = (line.find("{{ images/"), line.find(" }}")) {
                // {{ images/[id]_* }}
                let input_string = &line[s_idx..e_idx + 3];
                let id_s = input_string.find("/").expect("Image references must be / prefixed");
                let id_e = input_string.find("_").expect("Image references must be id prefixed");
    
                let id = String::from(&input_string[id_s+1..id_e]);
    
                let image_name = image_names.get(&id).expect(&format!("Invalid image reference {}", id));
    
                let image_ref_string = format!("</div>\n<img src={{\"/build/{post_name}/images/{image_name}\"}}/>\n<div>");
    
                line.replace_range(s_idx..e_idx + 3, &image_ref_string);
            }
            else if let (Some(s_idx), Some(e_idx)) = (line.find("{{ videos/"), line.find(" }}")) {
                // {{ videos/[id]_* }}
                let input_string = &line[s_idx..e_idx + 3];
                let id_s = input_string.find("/").expect("Image references must be / prefixed");
                let id_e = input_string.find("_").expect("Image references must be id prefixed");
    
                let id = String::from(&input_string[id_s+1..id_e]);
    
                let image_name = image_names.get(&id).expect("Invalid image reference");

                let image_ref_string = format!("<source src={{\"/build/{post_name}/images/{image_name}\"}} type=\"video/webm\"/>");
    
                let image_ref_string = format!("</div><video autoplay=true width=500 loop=true>\n{image_ref_string}\n</video><div>");
    
                line.replace_range(s_idx..e_idx + 3, &image_ref_string);
            }
            // remove all the ``` thingys and replace them with <pre><code> and </pre></code> if we're in a code block
            else if line == "```" {
                if code_block {
                    line = "\"# }}</code></pre>".to_string();
                } else {
                    line = "<pre><code>{{ r#\"".to_string();
                }
                code_block = !code_block;
            }
            else {
                if !code_block {
                    if line.starts_with("#") {
                        line = format!("<h1>{{ r#\"{}\"# }}</h1>", &line[2..]);
                    } else {
                        line = format!("<div>{{ r#\"{}\"# }}</div>", line);
                    }
                }
            }
            // replace all the `value` quoted things and replace them with <pre>value</pre>
            post_string.push_str(&line.clone());
            post_string.push_str("\n");
        }

        post_string = format!("<span markdown=\"block\" style=\"white-space: pre-wrap\"><div markdown=\"span\">\n{post_string}</div></span>");

        blog_strings.insert(post_name.clone(), post_string.clone());
        // println!("post name: {}", post_name);
        // for key in image_names.keys() {
            // println!("img_{}: {}", key.clone(), image_names.get(key).unwrap().clone());
        // }
    }


    std::fs::create_dir_all("src/generated").unwrap();
    let mut output = File::create("src/generated/posts.rs").unwrap();

    let mut file_string = String::new();
    file_string.push_str("use yew::prelude::*;\n");
    file_string.push_str("use std::collections::HashMap;\n");
    file_string.push_str("pub struct Posts { pub posts: HashMap::<String, Html> }\n");
    file_string.push_str(r"impl Posts {
    pub fn new() -> Self {
        let mut post_map = HashMap::<String, Html>::new();
    ");
    
    for blog in blog_strings.keys() {
        let html_macro = r"html! {";
        let html_macro_end = r"}";
        let blog_string = format!("{}", blog_strings.get(blog).unwrap());
        file_string.push_str(format!("\tpost_map.insert(String::from(\"{}\"), ", blog).as_str());
        file_string.push_str(format!("{html_macro}{}{html_macro_end}", blog_string).as_str());
        file_string.push_str(");");
    }

    file_string.push_str(r"
        Self {
            posts: post_map,
        }
    }
}");

    let _ = write!(output, "{}", file_string);

    let mut mod_file = File::create("src/generated/mod.rs").unwrap();
    let _ = write!(mod_file, "{}", "pub mod posts;");

}
