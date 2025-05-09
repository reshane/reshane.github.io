use wasm_bindgen::prelude::*;
use askama::Template;
use askama_markdown_cmark::filters;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response};

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub title: String,
    pub contents: String,
}

mod site_data;
use crate::site_data::*;

#[wasm_bindgen]
pub async fn fetch_post(post_title: &str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let url = format!("/posts/{}/index.md", post_title);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "text/markdown")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let markdown = JsFuture::from(resp.text()?).await?;

    Ok(markdown)
}

pub async fn get_post(post_name: &str) -> Result<PostTemplate, ()> {
    if let Ok(markdown) = fetch_post(post_name).await {
        match serde_wasm_bindgen::from_value(markdown) {
            Ok(post) => {
                Ok(PostTemplate {
                    title: post_name.replace(".md", "").to_string(),
                    contents: post,
                })
            },
            Err(e) => {
                console::error_1(&format!("{:?}", e).into());
                Err(())
            }
        }
    } else { Err(()) }
}

#[wasm_bindgen]
pub async fn render_post(post_name: String) {
    let post = get_post(post_name.as_str()).await;
    let container = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("post_container")
        .unwrap();
    container.replace_children_with_node_0();
    if let Ok(post) = post {
        let post_element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        post_element.set_inner_html(post.render().unwrap().as_str());
        post_element.set_class_name("post-contents");
        post_element.set_id(post.title.as_str());
        container.append_child(&post_element).unwrap();
    }
}

#[wasm_bindgen]
pub fn render_demos() {
    let demo_list = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("demo_list")
        .unwrap();
    for name in DEMO_NAMES {
        let demo_element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("li")
            .unwrap();
        let demo_link = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("a")
            .unwrap();
        demo_link.set_inner_html(&name);
        demo_link.set_attribute("href", format!("demos/{name}").as_str()).unwrap();
        demo_element.append_child(&demo_link).unwrap();
        demo_list.append_child(&demo_element).unwrap();
    }
}

#[wasm_bindgen]
pub fn render_links() {
    let post_list = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("post_list")
        .unwrap();
    for name in POST_NAMES {
        let post_element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("li")
            .unwrap();
        let post_link = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("a")
            .unwrap();
        post_link.set_inner_html(&name);
        post_link.set_attribute("href", format!("posts/{name}").as_str()).unwrap();
        post_element.append_child(&post_link).unwrap();
        post_list.append_child(&post_element).unwrap();
    }
}

