use yew::prelude::*;
use std::collections::HashMap;
pub struct Posts { pub posts: HashMap::<String, Html> }
impl Posts {
    pub fn new() -> Self {
        let mut post_map = HashMap::<String, Html>::new();
    	post_map.insert(String::from("image_resizing_20241106"), html! {<span markdown="block" style="white-space: pre-wrap"><div markdown="span">
<h1>{ r#"Image resizing blog"# }</h1>
<div>{ r#""# }</div>
<div>{ r#"Start by creating cargo project"# }</div>
</div>
<img src={"/build/image_resizing_20241106/images/1_aw_nutz.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#"copy shell.nix from another project & start the shell"# }</div>
<div>{ r#""# }</div>
<div>{ r#"where is my fish?"# }</div>
</div>
<img src={"/build/image_resizing_20241106/images/2_where_my_fish.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#"exit that shell"# }</div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/image_resizing_20241106/images/3_exit_dirty_bash.webm"} type="video/webm"/>
</video><div>
<div>{ r#""# }</div>
<div>{ r#"look up youtube video"# }</div>
<div>{ r#" - https://www.youtube.com/watch?v=tv9s4jhdUpU"# }</div>
<div>{ r#""# }</div>
<div>{ r#"come back in a few days and start writing the resizer again"# }</div>
<div>{ r#"So far, this is what we have in main.rs:"# }</div>
<pre><code>{{ r#"
use axum::{
    routing::get,
    body::Bytes,
    http::StatusCode,
    extract::Query,
    Router,
    debug_handler,
};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct Task {
    width: Option<u32>,
    height: Option<u32>,
    url: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt()
        // .compact().with_file(true)
        // .with_line_number(true)
        // .with_thread_ids(true)
        // .with_target(true).finish();
    let sub = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(sub).unwrap();


    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/resize", get(resize));
    info!("starting");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
#[debug_handler]
async fn resize(task: Query<Task>) -> (StatusCode, Bytes) {
    // we need to fetch the image here
    info!(
        "Received request: url: {:?} w: {:?} h: {:?}",
        task.url, task.width, task.height
    );
    (StatusCode::OK, task.url.as_bytes().to_vec().into())
}
"# }}</code></pre>
<div>{ r#"when we run it, we get this in the logs: "# }</div>
</div>
<img src={"/build/image_resizing_20241106/images/4_create_project.png"}/>
<div>
<div>{ r#"and we get this back when we send a request:"# }</div>
</div>
<img src={"/build/image_resizing_20241106/images/5_first_response.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#"Now, we'll write a module that fetches images for us and returns them in a DynamicImage"# }</div>
<div>{ r#"First, we'll restructure what we have & create files for our image fetching module"# }</div>
<div>{ r#"It'll live with all of our http-client-like things, so we'll name the module http"# }</div>
<div>{ r#"We'll also introduce an error module to make using the `?` easier."# }</div>
<div>{ r#"And just like that our structure looks like this:"# }</div>
</div>
<img src={"/build/image_resizing_20241106/images/6_new_file_structure.png"}/>
<div>
<div>{ r#"And our `src/error.rs` looks like this:"# }</div>
<pre><code>{{ r#"
// errors for the service

use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
}

"# }}</code></pre>
<div>{ r#"Note that we also added the `derive_more` crate. (`cargo add derive_more --features full`)"# }</div>
<div>{ r#"This will come into play later when converting from external errors to our service errors."# }</div>
<div>{ r#""# }</div>
<div>{ r#"Now, we'll start converting all of our `unwrap()`s into `?` operators."# }</div>
<div>{ r#"Starting in lib.rs, we make `run` return a `Result<()>` and..."# }</div>
<pre><code>{{ r#"
error[E0277]: `?` couldn't convert the error to `error::Error`
  --> src/lib.rs:50:37
   |
50 |     axum::serve(listener, app).await?;
   |                                     ^ the trait `From<std::io::Error>` is not implemented for `error::Error`, which is required by `std::result::Result<(), error::Error>: FromResidual<std::result::Result<Infallible, std::io::Error>>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the trait `FromResidual<std::result::Result<Infallible, E>>` is implemented for `std::result::Result<T, F>`
   = note: required for `std::result::Result<(), error::Error>` to implement `FromResidual<std::result::Result<Infallible, std::io::Error>>`
"# }}</code></pre>
<div>{ r#"now it yells at us to make our error implement `From` for `std::io::Error`."# }</div>
<div>{ r#"This should be easy with `derive_more`."# }</div>
<pre><code>{{ r#"
#[derive(From, Debug)]
pub enum Error {
    // External
    #[from]
    TracingGlobalDefault(tracing::subscriber::SetGlobalDefaultError),
    #[from]
    ServerBinding(std::io::Error),
}
"# }}</code></pre>
<div>{ r#"And no more errors! We'll need to add more to this enum while implementing our image fetcher..."# }</div>
<div>{ r#"So we'll do that now"# }</div>
<div>{ r#"We want to be caching as much as possible in order to speed this thing up because image processing pretty slow."# }</div>
<div>{ r#"In order to fetch images faster, we should actually send two requests - "# }</div>
<div>{ r#"    1. Grab the data from the provided image url"# }</div>
<div>{ r#"    2. Grab the data from a local image store if possible"# }</div>
<div>{ r#"Typically this would be done in order, but we can use tokio to just fire off two requests and take whichever comes back successfully first."# }</div>
<div>{ r#"I'm not certain this is actually a performance improvement for these reasons:"# }</div>
<div>{ r#"    - Requesting a local image that doesn't exist will 404 very quickly anyway"# }</div>
<div>{ r#"    - Always sending off two requests rather than sometimes only sending off one could be detrimental under heavy load"# }</div>
<div>{ r#"I'm going to do it anyway for these reasons:"# }</div>
<div>{ r#"    - Learning experience"# }</div>
<div>{ r#"    - Sometimes you have to get fancy and confusing to discover simplicity"# }</div>
<div>{ r#"    - Gives me an opportunity to find out which is faster later"# }</div>
<div>{ r#"So, I'll be creating two different implementations of our image fetching function"# }</div>
<div>{ r#"`serial_fetch`"# }</div>
<div>{ r#"`parallel_fetch`"# }</div>
<div>{ r#"Lets get into it..."# }</div>
<div>{ r#""# }</div>
<div>{ r#"In order to write a simple http client we'll use hyper, since axum already uses it"# }</div>
<div>{ r#"And I'll be starting from this example: `https://hyper.rs/guides/1/client/basic/`"# }</div>
<div>{ r#"Making a single request to begin with, we have something like this:"# }</div>
<pre><code>{{ r#"
#[tracing::instrument]
pub async fn serial_fetch(url: String) -> Result<DynamicImage> {
    // parse our url
    let url = url.parse::<hyper::Uri>()?;

    // get the host and the port
    let host = url.host().unwrap();
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);

    // Open TCP connection to the remote host
    let stream = TcpStream::connect(address).await?;

    let io = TokioIo::new(stream);

    // create the Hyper Client
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    // spawn a task to poll the connection, driving the http state
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            error!("Connection failed: {:?}", err);
        };
    });

    // the authority of our url will be the host
    let authority = url.authority().unwrap().clone();

    // create an http request with an empty body & host header
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    // await the response
    let mut res = sender.send_request(req).await?;

    let mut image_bytes = Vec::<u8>::new();

    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            image_bytes.extend(chunk);
        }

    }

    let mut img = ImageReader::new(std::io::Cursor::new(image_bytes));
    img.set_format(ImageFormat::Png);
    let img = img.decode()?;

    Ok(img)

}
"# }}</code></pre>
<div>{ r#"Some TODO's for this thing:"# }</div>
<div>{ r#"    - abstract away the 'client' construct"# }</div>
<div>{ r#"    - parsing the bytes into an image should be separate"# }</div>
<div>{ r#"        - this is going to get more complicated with more formats"# }</div>
<div>{ r#""# }</div>
<div>{ r#"For now, we can start using this thing by updating our handler in lib.rs like this:"# }</div>
<pre><code>{{ r#"
async fn resize(task: Query<Task>) -> impl IntoResponse {
    // we need to fetch the image here
    info!(
        "Received request: url: {:?} w: {:?} h: {:?}",
        task.url, task.width, task.height
    );
    match image_fetcher::serial_fetch(task.url.clone()).await {
        Ok(img) => {
            let mut bytes = Vec::<u8>::new();
            img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
            return (
                StatusCode::OK,
                AppendHeaders([
                    ("Content-Type", "image/png")
                ]),
                bytes.into()
            );
        },
        Err(err) => {
            error!("Could not get the image {:?}", err);
            return (
                StatusCode::NOT_FOUND,
                AppendHeaders([
                    ("Content-Type", "image/png"),
                ]),
                Bytes::new()
            );
        },
    }
}
"# }}</code></pre>
<div>{ r#"For convenience, we write a little test script `test_curl`:"# }</div>
<pre><code>{{ r#"
#! /bin/sh

curl \
    "localhost:3000/resize?url=http%3A%2F%2Fstatic.wikia.nocookie.net%2Fadventuretimewithfinnandjake%2Fimages%2F9%2F9e%2FPeppermint_Butler.png" \
    --verbose \
    --output out.png
"# }}</code></pre>
<div>{ r#"and then we can `cargo r` and `./test_curl` to see if it works"# }</div>
</div>
<img src={"/build/image_resizing_20241106/images/7_test_curl_logs.png"}/>
<div>
</div>
<img src={"/build/image_resizing_20241106/images/8_test_curl_output.png"}/>
<div>
<div>{ r#"Great!"# }</div>
<div>{ r#"Now to actually resize the image..."# }</div>
<div>{ r#"This functionality should definitely be put into another module - we're no longer dealing with http requests."# }</div>
<div>{ r#"So we create a new module img which will contain all of our image parsing and manipulation stuff"# }</div>
</div>
<img src={"/build/image_resizing_20241106/images/9_new_img_module.png"}/>
<div>
<div>{ r#"And insize `img/resizer.rs` we can add something like this to get it resizing images..."# }</div>
<pre><code>{{ r#"
// image manipulation functionality

use image::DynamicImage;

pub async fn resize(image: &mut DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
}
"# }}</code></pre>
<div>{ r#"And then update our `resize` handler in lib.rs to call to a function that uses our new `resizer::resize` function:"# }</div>
<pre><code>{{ r#"
async fn handle_resize(task: Task) -> Result<Vec<u8>> {
    match image_fetcher::serial_fetch(task.url.clone()).await {
        Ok(img) => {
            let img = resizer::resize(
                &mut img.clone(), 
                task.width.expect("NO WIDTH FOUND!"),
                task.height.expect("NO HEIGHT FOUND!")
            ).await;
            let mut bytes = Vec::<u8>::new();
            img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
            return Ok(bytes);
        },
        Err(err) => {
            error!("Could not get the image {:?}", err);
            return Err(err)
        },
    }
}

#[debug_handler]
async fn resize(Query(task): Query<Task>) -> impl IntoResponse {
    // we need to fetch the image here
    info!(
        "Received request: url: {:?} w: {:?} h: {:?}",
        task.url, task.width, task.height
    );
    match handle_resize(task.into()).await {
        Ok(bytes) => {
            (
                StatusCode::OK,
                AppendHeaders([
                    ("Content-Type", "image/png")
                ]),
                bytes.into()
            )
        },
        Err(_err) => {
            (
                StatusCode::NOT_FOUND,
                AppendHeaders([
                    ("Content-Type", "image/png"),
                ]),
                Bytes::new()
            )
        },
    }
}
"# }}</code></pre>
<div>{ r#"And now we can update our `test_curl` script to include a `width` and `height` parameters"# }</div>
<div>{ r#"And just like that we have a slightly wider peppermint butler"# }</div>
</div>
<img src={"/build/image_resizing_20241106/images/10_wide_pep_but.png"}/>
<div>
<div>{ r#""# }</div>
</div></span>});
        Self {
            posts: post_map,
        }
    }
}