use yew::prelude::*;
use std::collections::HashMap;
pub struct Posts { pub posts: HashMap::<String, Html> }
impl Posts {
    pub fn new() -> Self {
        let mut post_map = HashMap::<String, Html>::new();
    	post_map.insert(String::from("20241106_image_resizing"), html! {<span markdown="block" style="white-space: pre-wrap"><div markdown="span">
<h1>{ r#"Image resizing blog"# }</h1>
<div>{ r#""# }</div>
<div>{ r#"Start by creating cargo project"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/1_aw_nutz.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#"copy shell.nix from another project & start the shell"# }</div>
<div>{ r#""# }</div>
<div>{ r#"where is my fish?"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/2_where_my_fish.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#"exit that shell"# }</div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241106_image_resizing/images/3_exit_dirty_bash.webm"} type="video/webm"/>
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
<img src={"/build/20241106_image_resizing/images/4_create_project.png"}/>
<div>
<div>{ r#"and we get this back when we send a request:"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/5_first_response.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#"Now, we'll write a module that fetches images for us and returns them in a DynamicImage"# }</div>
<div>{ r#"First, we'll restructure what we have & create files for our image fetching module"# }</div>
<div>{ r#"It'll live with all of our http-client-like things, so we'll name the module http"# }</div>
<div>{ r#"We'll also introduce an error module to make using the "# }<code>{ r#"?"# }</code>{ r#" easier."# }</div>
<div>{ r#"And just like that our structure looks like this:"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/6_new_file_structure.png"}/>
<div>
<div>{ r#"And our "# }<code>{ r#"src/error.rs"# }</code>{ r#" looks like this:"# }</div>
<pre><code>{{ r#"
// errors for the service

use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
}

"# }}</code></pre>
<div>{ r#"Note that we also added the "# }<code>{ r#"derive_more"# }</code>{ r#" crate. ("# }<code>{ r#"cargo add derive_more --features full"# }</code>{ r#")"# }</div>
<div>{ r#"This will come into play later when converting from external errors to our service errors."# }</div>
<div>{ r#""# }</div>
<div>{ r#"Now, we'll start converting all of our "# }<code>{ r#"unwrap()"# }</code>{ r#"s into "# }<code>{ r#"?"# }</code>{ r#" operators."# }</div>
<div>{ r#"Starting in lib.rs, we make "# }<code>{ r#"run"# }</code>{ r#" return a "# }<code>{ r#"Result<()>"# }</code>{ r#" and..."# }</div>
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
<div>{ r#"now it yells at us to make our error implement "# }<code>{ r#"From"# }</code>{ r#" for "# }<code>{ r#"std::io::Error"# }</code>{ r#"."# }</div>
<div>{ r#"This should be easy with "# }<code>{ r#"derive_more"# }</code>{ r#"."# }</div>
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
<div>{ r#""# }<code>{ r#"serial_fetch"# }</code>{ r#""# }</div>
<div>{ r#""# }<code>{ r#"parallel_fetch"# }</code>{ r#""# }</div>
<div>{ r#"Lets get into it..."# }</div>
<div>{ r#""# }</div>
<div>{ r#"In order to write a simple http client we'll use hyper, since axum already uses it"# }</div>
<div>{ r#"And I'll be starting from this example: "# }<code>{ r#"https://hyper.rs/guides/1/client/basic/"# }</code>{ r#""# }</div>
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
<div>{ r#"For convenience, we write a little test script "# }<code>{ r#"test_curl"# }</code>{ r#":"# }</div>
<pre><code>{{ r#"
#! /bin/sh

curl \
    "localhost:3000/resize?url=http%3A%2F%2Fstatic.wikia.nocookie.net%2Fadventuretimewithfinnandjake%2Fimages%2F9%2F9e%2FPeppermint_Butler.png" \
    --verbose \
    --output out.png
"# }}</code></pre>
<div>{ r#"and then we can "# }<code>{ r#"cargo r"# }</code>{ r#" and "# }<code>{ r#"./test_curl"# }</code>{ r#" to see if it works"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/7_test_curl_logs.png"}/>
<div>
</div>
<img src={"/build/20241106_image_resizing/images/8_test_curl_output.png"}/>
<div>
<div>{ r#"Great!"# }</div>
<div>{ r#"Now to actually resize the image..."# }</div>
<div>{ r#"This functionality should definitely be put into another module - we're no longer dealing with http requests."# }</div>
<div>{ r#"So we create a new module img which will contain all of our image parsing and manipulation stuff"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/9_new_img_module.png"}/>
<div>
<div>{ r#"And insize "# }<code>{ r#"img/resizer.rs"# }</code>{ r#" we can add something like this to get it resizing images..."# }</div>
<pre><code>{{ r#"
// image manipulation functionality

use image::DynamicImage;

pub async fn resize(image: &mut DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
}
"# }}</code></pre>
<div>{ r#"And then update our "# }<code>{ r#"resize"# }</code>{ r#" handler in lib.rs to call to a function that uses our new "# }<code>{ r#"resizer::resize"# }</code>{ r#" function:"# }</div>
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
<div>{ r#"And now we can update our "# }<code>{ r#"test_curl"# }</code>{ r#" script to include a "# }<code>{ r#"width"# }</code>{ r#" and "# }<code>{ r#"height"# }</code>{ r#" parameters"# }</div>
<div>{ r#"And just like that we have a slightly wider peppermint butler"# }</div>
</div>
<img src={"/build/20241106_image_resizing/images/10_wide_pep_but.png"}/>
<div>
<div>{ r#""# }</div>
</div></span>});	post_map.insert(String::from("20241113_convolution"), html! {<span markdown="block" style="white-space: pre-wrap"><div markdown="span">
<h1>{ r#"Convolution"# }</h1>
<div>{ r#"I want to do some convolution stuff in c because it seems fun."# }</div>
<div>{ r#"So I start by making a project and taking some screenshots for this blog I guess."# }</div>
</div>
<img src={"/build/20241113_convolution/images/0_file_structure.png"}/>
<div>
</div>
<img src={"/build/20241113_convolution/images/1_initial_main_c.png"}/>
<div>
</div>
<img src={"/build/20241113_convolution/images/2_initial_convolve_h.png"}/>
<div>
</div>
<img src={"/build/20241113_convolution/images/3_initial_convolve_c.png"}/>
<div>
<div>{ r#"Now to figure out how to build all of it..."# }</div>
<div>{ r#""# }<a href={ "https://stackoverflow.com/questions/1705961/how-to-link-to-a-static-library-in-c" }>{ "To stack overflow!" }
</a>{ r#""# }</div>
<div>{ r#"And we end up with this:"# }</div>
</div>
<img src={"/build/20241113_convolution/images/4_build_script_initial.png"}/>
<div>
<div>{ r#"Which allows us to do this: "# }</div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241113_convolution/images/5_initial_run.webm"} type="video/webm"/>
</video><div>
<div>{ r#"Great! Now let's warm up with some light convolution before starting the crazy stuff..."# }</div>
<div>{ r#"For that I'm going to write a data structure to help out a little"# }</div>
</div>
<img src={"/build/20241113_convolution/images/6_int_vector.png"}/>
<div>
<div>{ r#"Admitedly, the naming of this thing is not great, but refactoring is a beautiful thing that I expect to do a lot of later."# }</div>
<div>{ r#"For now, we can just start implementing this."# }</div>
<div>{ r#"Most of it is pretty simple, malloc for our structure once and then malloc/realloc for our data once the capacity is exceeded..."# }</div>
<pre><code>{{ r#"
// vector

Vector_i* Vector_i_new() {
    Vector_i* new = (Vector_i*) malloc(sizeof(Vector_i));
    if (!new) {
        printf("ERROR: Could initialize vector!");
        exit(1);
    }
    new->size = 0;
    new->capacity = 0;
    new->data = NULL;
    return new;
}

void Vector_i_push(Vector_i* v, int e) {
    if (v->capacity == 0) {
        v->capacity = 1;
        v->data = (int*) malloc(v->capacity * sizeof(int));
        if (!v->data) exit(1);
    }
    else if (v->size == v->capacity) {
        v->capacity *= 2;
        int* newMem = (int*) realloc(v->data, v->capacity * sizeof(int));
        if (!newMem) {
            // we have an error here... uh-oh!
            printf("ERROR: Could not increase the size of the vector");
            exit(1);
        }
        v->data = newMem;
    }
    v->data[v->size++] = e;
}

int Vector_i_get(Vector_i* v, size_t idx) {
    assert(idx < v->size);
    return v->data[idx];
}

size_t Vector_i_size(Vector_i* v) {
    return v->size;
}

size_t Vector_i_capacity(Vector_i* v) {
    return v->capacity;
}

void Vector_i_free(Vector_i* v) {
    free(v->data);
    free(v);
}
"# }}</code></pre>
<div>{ r#"Also important to implement a free function so we aren't leaking a bunch of memory."# }</div>
<div>{ r#"Even though what I'm going to be using this for will pretty much exclusivly be short-lived programs, just in case I ever use it for something longer, it'll be nice to have this function."# }</div>
<div>{ r#"And best practices and all that stuff of course."# }</div>
<div>{ r#"We can also write a test for it"# }</div>
<div>{ r#""# }<a href={ "https://www.youtube.com/watch?v=5aZiRjgSGQU" }>{ "Thanks to Kay Lack for the little test framework" }
</a>{ r#""# }</div>
</div>
<img src={"/build/20241113_convolution/images/8_tests_initial.png"}/>
<div>
<div>{ r#"(ignore all that "# }<code>{ r#"Vector_f"# }</code>{ r#" stuff, that's for later and I forgot to remove it for the screenshot...)"# }</div>
<div>{ r#"Now for the "# }<code>{ r#"Vector_i_convolve"# }</code>{ r#" function..."# }</div>
<div>{ r#"I'll want to take in two vectors to perform the convolution over eachother, so we'll call them a and b"# }</div>
<div>{ r#"And basically what we'll be doing is something like this:"# }</div>
<pre><code>{{ r#"
                           a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
                        a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
                     a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
                  a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
               a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
            a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
         a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
      a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
   a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
   b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
      b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
         b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
            b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
               b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                  b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                     b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                        b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
a[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                           b[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
"# }}</code></pre>
<div>{ r#"where we can imagine flipping b around so that its lowest indices are on the right,"# }</div>
<div>{ r#"and its 0th index is placed under the 0th index of a"# }</div>
<div>{ r#"we can then multuply the elements that are 'facing eachother', and shift b to the right"# }</div>
<div>{ r#"Now, dealing with the pairs of operations should look something like this"# }</div>
<pre><code>{{ r#"
a[0] * b[0]
a[0] * b[1] + a[1] * b[0]
a[0] * b[2] + a[1] * b[1] + a[2] * b[0]
...
a[1] * b[9] + a[2] * b[8] * a[3] * b[7] ... 
a[2] * b[9] + a[3] * b[8] * a[4] * b[7] ... 
"# }}</code></pre>
<div>{ r#"And after lots of thought and effort, we can write this function:"# }</div>
<pre><code>{{ r#"
Vector_i* Vector_i_convolve(Vector_i* a, Vector_i* b) {
    Vector_i* result = Vector_i_new();

    int a_size = Vector_i_size(a);
    int b_size = Vector_i_size(b);

    int a_start = 0, b_start = 0;
    for (int i=0; i<a_size+b_size; ++i) {
        int j=a_start,k=b_start,total=0;
        while (j<a_size && k>-1) {
            total += Vector_i_get(a, j) * Vector_i_get(b, k);
            j++;
            k--;
        }
        Vector_i_push(result, total);
        if (b_start == b_size-1) {
            a_start++;
        } else {
            b_start++;
        }
    }

    return result;
}
"# }}</code></pre>
<div>{ r#"and then we can try it out in our main:"# }</div>
<div>{ r#"(Note that I have DEBUG in a "# }<code>{ r#"#define"# }</code>{ r#" statement above main to turn the printing on/off"# }</div>
<pre><code>{{ r#"
int main() {
    Vector_i* base = Vector_i_new();
    Vector_i* mask = Vector_i_new();

    for (int i = 0; i < SIZE; ++i) {
        Vector_i_push(base, i);
        if (DEBUG) printf("vector contents\n\tsize: %d\n\tcap: %d\n\tlast: %d\n", base->size, base->capacity, base->data[base->size-1]);
        Vector_i_push(mask, i);
        if (DEBUG) printf("vector contents\n\tsize: %d\n\tcap: %d\n\tlast: %d\n", mask->size, mask->capacity, mask->data[mask->size-1]);
    }

    if (DEBUG) printf("vector contents\n\tsize: %d\n\tcap: %d\n\tlast: %d\n", base->size, base->capacity, base->data[base->size-1]);
    if (DEBUG) printf("vector contents\n\tsize: %d\n\tcap: %d\n\tlast: %d\n", mask->size, mask->capacity, mask->data[mask->size-1]);

    Vector_i* result = Vector_i_convolve(base, mask);

    if (1) {
        printf("[");
        for (int i = 0; i < Vector_i_size(result); ++i) {
            printf("%d", Vector_i_get(result, i));
            if (i+1 != Vector_i_size(result)) printf(",");
        }
        printf("]");
    }

    Vector_i_free(base);
    Vector_i_free(mask);
    Vector_i_free(result);

    return 0;
}
"# }}</code></pre>
<div>{ r#"And with "# }<code>{ r#"DEBUG = 0"# }</code>{ r#", when we "# }<code>{ r#"./build.sh && ./build/main"# }</code>{ r#""# }</div>
</div>
<img src={"/build/20241113_convolution/images/7_vector_out_initial.png"}/>
<div>
<div>{ r#"Great! Thats probably right... "# }</div>
<div>{ r#"Lets start in the convolve.h file by defining our interface:"# }</div>
<pre><code>{{ r#"
// matrix

typedef struct {
    size_t rows;
    size_t cols;
    int* data;
} Matrix_i;

Matrix_i* Matrix_i_new(size_t rows, size_t cols);

size_t Matrix_i_rows(Matrix_i* m);

size_t Matrix_i_cols(Matrix_i* m);

void Matrix_i_set(Matrix_i* m, size_t x, size_t y, int e);

int Matrix_i_get(Matrix_i* m, size_t x, size_t y);

void Matrix_i_free(Matrix_i* m);

Matrix_i* Matrix_i_convolve(Matrix_i* a, Matrix_i* b);
"# }}</code></pre>
<div>{ r#"Now we can implement our functions, pretty much the same way we did for our "# }<code>{ r#"Vector_i"# }</code>{ r#", which I will skip for brevity"# }</div>
<div>{ r#"So once we have all our helper functions we can start convolving matrices."# }</div>
<div>{ r#"This is going to be a lot simpler than the vector convolution because for the vector we did a biblically accurate convolution - where it is mathematically correct."# }</div>
<div>{ r#"For the matrix, we are essentially going to do the dumbed-down computer science version & just take a moving weighted sum of each element."# }</div>
<div>{ r#"I am also going to ignore all of the little edges and essentially treat the middle of our matrix b as the sole point of focus."# }</div>
<div>{ r#"And calculate an output for each element of a with b positioned such that its center is paired with the element of a in question."# }</div>
<div>{ r#"So, for example - If we were doing a biblically accurate matrix convolution, we would have to deal with cases like this:"# }</div>
<pre><code>{{ r#"
 _________
|         |
|         |
|    b    |
|        _|_______
|_______|_|       |
        |         |
        |    a    |
        |         |
        |_________|
"# }}</code></pre>
<div>{ r#"In the case where we are convolving b over a, above is the visual representation of the first step"# }</div>
<div>{ r#"where the overlap represents elements of each matrix that are 'paired up' for this iteration,"# }</div>
<div>{ r#"and therefore should be multiplied pair-wise and then summed to produce the output."# }</div>
<div>{ r#"If we were to do this, the resulting matrix would be larger than a..."# }</div>
<div>{ r#"How much larger? I'm not sure but we can figure that out now (since I don't feel like writing the function right now)"# }</div>
<div>{ r#"So we start with the matrices positioned as above."# }</div>
<div>{ r#"We perform an operation, shift, and repeat until we find a and b in the following configuration"# }</div>
<pre><code>{{ r#"
         _________
        |         |
        |         |
        |    b    |
        |_________|
        |_________|
        |         |
        |    a    |
        |         |
        |_________|
"# }}</code></pre>
<div>{ r#"In the case above b and a have the same width, but regardless of relative widths, we will have done "# }<code>{ r#"Matrix_i_cols(a)"# }</code>{ r#" operations and therefore have generated as many outputs."# }</div>
<div>{ r#"Then, we continue until just a single element of each is overlapping, like this"# }</div>
<pre><code>{{ r#"
                 _________
                |         |
                |         |
                |    b    |
         _______|_        |
        |       |_|_______|
        |         |
        |    a    |
        |         |
        |_________|
"# }}</code></pre>
<div>{ r#"So we've now done an additional "# }<code>{ r#"Matrix_i_cols(b) - 1"# }</code>{ r#" operations, and thus have generated as many outputs"# }</div>
<div>{ r#"Which puts us at "# }<code>{ r#"Matrix_i_cols(a) + Matrix_i_cols(b) - 1"# }</code>{ r#" "# }</div>
<div>{ r#"Flipping the whole thing on it's side we can see that the same holds true for the height"# }</div>
<div>{ r#"And so the biblically accurate output matrix has the following dimensions:"# }</div>
<div>{ r#"width: "# }<code>{ r#"Matrix_i_cols(a) + Matrix_i_cols(b) - 1"# }</code>{ r#""# }</div>
<div>{ r#"height: "# }<code>{ r#"Matrix_i_rows(a) + Matrix_i_rows(b) - 1"# }</code>{ r#""# }</div>
<div>{ r#"Anyway, now lets write the blasphemers version"# }</div>
<div>{ r#"For each element in a, we apply b as if its center is at the current element of a."# }</div>
<div>{ r#"Applying b involves pairing each element with an element from a by its relative position."# }</div>
<pre><code>{{ r#"
Matrix_i* Matrix_i_convolve(Matrix_i* a, Matrix_i* b) {
    size_t a_w = Matrix_i_cols(a);
    size_t a_h = Matrix_i_rows(a);

    size_t b_w = Matrix_i_cols(b);
    size_t b_h = Matrix_i_rows(b);

    Matrix_i* result = Matrix_i_new(a_w, a_h);

    for (size_t y=0; y<a_h; ++y) {
        for (size_t x=0; x<a_w; ++x) {
            uint8_t bytes[4] = {0, 0, 0, 0};
            int weighted_sum = 0;
            for (size_t i=0; i<b_h; ++i) {
                for (size_t j=0; j<b_w; ++j) {
                    int c = Matrix_i_get(b,i,j);
                    int a_x = ((x+j)-(b_w/2));
                    int a_y = ((y+i)-(b_h/2));

                    if ((a_x>-1) && (a_x<a_w) && (a_y>-1) && (a_y<a_h)) {
                        weighted_sum += Matrix_i_get(a,a_x,a_y) * c;
                    }
                }
            }
            Matrix_i_set(result,y,x, weighted_sum);
        }
    }
    return result;
}
"# }}</code></pre>
<div>{ r#"Here, we are calculating the index into a by adding the current index of a and b & subtracting half of the corresponding dimension of b."# }</div>
<div>{ r#"Now we can see if this thing is working..."# }</div>
</div>
<img src={"/build/20241113_convolution/images/9_matrix_convolution_initial.ong"}/>
<div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241113_convolution/images/10_matrix_output_initial.webm"} type="video/webm"/>
</video><div>
<div>{ r#"This doesn't really do anything for us though, what is the application for a discreet finite convolution of two integer matrices?"# }</div>
<div>{ r#"I'm sure someone smarter than me could find a use... but I'm not smarter than me yet"# }</div>
<div>{ r#"But what we can do is create new matrix types for unsigned 32-bit integers & floats, then convolve the floats over the ints"# }</div>
<div>{ r#"We can then encode image data in the "# }<code>{ r#"uint32_t"# }</code>{ r#" matrix and applying the convolution will change the image"# }</div>
<div>{ r#"So lets write that function:"# }</div>
<pre><code>{{ r#"
// cross-type convolution
Matrix_u32* Matrix_u32_f_convolve(Matrix_u32* a, Matrix_f* b) {
    size_t a_w = Matrix_u32_cols(a);
    size_t a_h = Matrix_u32_rows(a);

    size_t b_w = Matrix_f_cols(b);
    size_t b_h = Matrix_f_rows(b);

    Matrix_u32* result = Matrix_u32_new(a_w, a_h);

    for (size_t y=0; y<a_h; ++y) {
        for (size_t x=0; x<a_w; ++x) {
            uint32_t weighted_sum = 0;
            for (size_t i=0; i<b_h; ++i) {
                for (size_t j=0; j<b_w; ++j) {
                    float c = Matrix_f_get(b,i,j);
                    int a_x = ((x+j)-(b_w/2));
                    int a_y = ((y+i)-(b_h/2));

                    uint32_t pixel = Matrix_u32_get(a,x,y);
                    if ((a_x>-1) && (a_x<a_w) && (a_y>-1) && (a_y<a_h)) {
                        pixel = Matrix_u32_get(a,a_x,a_y);
                    }
                    // apply the mask to get the correct bytes
                    // shift them over to the right, multuply by the coefficient
                    // shift them back to where the were and add to the sum
                    weighted_sum += ((uint8_t)(((pixel&0x0000FF)>>8*0) * c))<<8*0;
                    weighted_sum += ((uint8_t)(((pixel&0x00FF00)>>8*1) * c))<<8*1;
                    weighted_sum += ((uint8_t)(((pixel&0xFF0000)>>8*2) * c))<<8*2;
                }
            }
            Matrix_u32_set(result,y,x, weighted_sum);
        }
    }
    return result;
}

"# }}</code></pre>
<div>{ r#"now this compiles, and we can use it in our main function..."# }</div>
<pre><code>{{ r#"
int main() {
    Matrix_u32* img = Matrix_u32_new(400, 400);
    Matrix_f* msk = Matrix_f_new(4, 4);
    tv_static(img);
    flat_blur_matrix(msk);
    save_img_as_ppm(img, "original.ppm");
    Matrix_u32* blr = Matrix_u32_f_convolve(img, msk);
    save_img_as_ppm(blr, "blurred.ppm");
    return 0;
}
"# }}</code></pre>
<div>{ r#"The first thing we do above is create a 400x400 matrix to represent out image."# }</div>
<div>{ r#"Then, we create a 4x4 mask that will be the box which we move over our image to blur it."# }</div>
<div>{ r#"Then we fill the image with either black or white pixels by passing its pointer to the "# }<code>{ r#"tv_static"# }</code>{ r#" function, which looks like this:"# }</div>
<pre><code>{{ r#"
void tv_static(Matrix_u32* img) {
    size_t w = Matrix_u32_cols(img);
    size_t h = Matrix_u32_rows(img);

    for (int y=0; y<h; ++y) {
        for (int x=0; x<w; ++x) {
            uint32_t pixel = 0xFF000000;
            if ((int)((rand()/(float)RAND_MAX) + 0.5)) {
                pixel = 0xFFFFFF;
            }
            Matrix_u32_set(img,x,y,pixel);
        }
    }
}
"# }}</code></pre>
<div>{ r#"Then we can fill up our msk matrix with values that sum to 1.0f with a function like this:"# }</div>
<pre><code>{{ r#"
void flat_blur_matrix(Matrix_f* msk) {
    size_t msk_w = Matrix_f_cols(msk);
    size_t msk_h = Matrix_f_rows(msk);
    for (int y=0; y<msk_h; ++y) {
        for (int x=0; x<msk_w; ++x) {
            Matrix_f_set(msk,y,x,1.0/(msk_w*msk_h));
        }
    }
}
"# }}</code></pre>
<div>{ r#"Then, I use this "# }<code>{ r#"save_image_as_ppm"# }</code>{ r#" to output the data into an image file with the given name, PPM being the choice of format because it's header is extremely simple:"# }</div>
<pre><code>{{ r#"
P6
WIDTH HEIGHT 255
RGB_IMAGE_BYTES
"# }}</code></pre>
<div>{ r#"And this is the function used to generate that file:"# }</div>
<pre><code>{{ r#"
void save_img_as_ppm(Matrix_u32* img, const char *file_path) {
    FILE* f = fopen(file_path, "wb");
    if (f == NULL) {
        fprintf(stderr, "ERROR: could not write into file %s: %s\n", file_path, strerror(errno));
        exit(1);
    }
    size_t img_w = Matrix_u32_cols(img);
    size_t img_h = Matrix_u32_rows(img);
    fprintf(f, "P6\n%d %d 255\n", img_w, img_h);
    for (size_t y=0; y<img_h; ++y) {
        for (size_t x=0; x<img_w; ++x) {
            uint32_t pixel = Matrix_u32_get(img,x,y);
            uint8_t bytes[3] = {
                (pixel&0x0000FF)>>8*0,
                (pixel&0x00FF00)>>8*1,
                (pixel&0xFF0000)>>8*2
            };
            fwrite(bytes, sizeof(bytes), 1, f);
            assert(!ferror(f));
        }
    }
    fclose(f);
}
"# }}</code></pre>
<div>{ r#"Then we apply our blur to the matrix, generating a new image data matrix, and call the same function on it to save it as a PPM image file."# }</div>
<div>{ r#"And just like that, we have our original.ppm:"# }</div>
</div>
<img src={"/build/20241113_convolution/images/11_original.png"}/>
<div>
<div>{ r#"And our blurred.ppm image:"# }</div>
</div>
<img src={"/build/20241113_convolution/images/12_blurred.png"}/>
<div>
<div>{ r#""# }</div>
<div>{ r#""# }</div>
</div></span>});	post_map.insert(String::from("20241125_fourier_transforms"), html! {<span markdown="block" style="white-space: pre-wrap"><div markdown="span">
<h1>{ r#"Fourier Transforms"# }</h1>
<div>{ r#""# }</div>
<div>{ r#"As I am someone who enjoys interesting algorithms, math, and re-implementing existing things, it was only a matter of time before I wrote a fourier transform."# }</div>
<div>{ r#"The first time I had the idea to do so was in college when I saw "# }<a href={ "" }>{ "this 3b1b video" }
</a>{ r#" so thats where we'll start"# }</div>
<div>{ r#"This time around I wanted to implement the algorithm within an stb-style header only library, so we'll start with some data structures to support our algorithm there..."# }</div>
<pre><code>{{ r#"
#ifndef FFT_H_
#define FFT_H_

// == INCLUDES ==

#include <stdio.h>
#include <math.h>
#include <complex.h>

#ifndef FFT_NO_UNI_STD
#include <unistd.h>
#endif // FFT_NO_UNI_STD

#ifndef FFT_MALLOC
#include <stdlib.h>
#define FFT_ALLOC malloc
#define FFT_FREE free
#endif // FFT_MALLOC

#ifndef FFT_ASSERT
#include <assert.h>
#define FFT_ASSERT assert
#endif // FFT_ASSERT


// == DEFINITIONS ==

// [[ CONSTANTS ]]

#define FFT_EPSILON 0.00001f

// [[ STRUCTURES ]]

// -- Vector --

typedef struct {
    size_t size;
    float complex* data;
} fft_Vec_cf;
fft_Vec_cf fft_vec_alloc(size_t cap);
void fft_vec_free(fft_Vec_cf* v);
// deprecated
float complex* fft_vec_at(fft_Vec_cf* v, size_t i);
float complex fft_vec_get(fft_Vec_cf* v, size_t i);
void fft_vec_set(fft_Vec_cf* v, size_t i, float complex e);

// -- Matrix --
typedef struct {
    size_t rows;
    size_t cols;
    float complex* data;
} fft_Matrix_cf;
fft_Matrix_cf fft_mat_alloc(size_t rows, size_t cols);
void fft_mat_free(fft_Matrix_cf* mat);
float complex fft_mat_get(fft_Matrix_cf* mat, size_t x, size_t y);
void fft_mat_set(fft_Matrix_cf* mat, size_t x, size_t y, float complex e);
"# }}</code></pre>
<div>{ r#"Along with all of our includes (some made optional by putting their functions behind macros which I thought was a cool way to provide configuration to users of the library),"# }</div>
<div>{ r#"We define vector and matrix data structures along with some associated functions"# }</div>
<div>{ r#"Also defined in our "# }<code>{ r#"[[ CONSTANTS ]]"# }</code>{ r#" section is an epsilon value, which will come in handy later for floating-point stuff..."# }</div>
<div>{ r#"Now we can move on to the implementation of the boilerplate for our data structures"# }</div>
<pre><code>{{ r#"
#ifdef FFT_IMPLEMENTATION

// == IMPLEMENTATION ==

// [[ STRUCTURES ]]

// -- Vector --

fft_Vec_cf fft_vec_alloc(size_t cap)
{
    fft_Vec_cf v;

    v.size = cap;
    v.data = FFT_ALLOC(v.size*sizeof(*v.data));
    FFT_ASSERT(v.data);

    return v;
}

void fft_vec_free(fft_Vec_cf* v)
{
    FFT_FREE(v->data);
}

float complex* fft_vec_at(fft_Vec_cf* v, size_t i)
{
    FFT_ASSERT(i < v->size);
    return &v->data[i];
}

float complex fft_vec_get(fft_Vec_cf* v, size_t i)
{
    return *fft_vec_at(v, i);
}

void fft_vec_set(fft_Vec_cf* v, size_t i, float complex e)
{
    FFT_ASSERT(i < v->size);
    *fft_vec_at(v, i) = e;
}

// -- Matrix --

fft_Matrix_cf fft_mat_alloc(size_t rows, size_t cols)
{
    fft_Matrix_cf mat;
    mat.rows = rows;
    mat.cols = cols;
    mat.data = (float complex*)malloc(sizeof(float complex)*rows*cols);
    FFT_ASSERT(mat.data);
    return mat;
}

void fft_mat_free(fft_Matrix_cf* mat)
{
    free(mat->data);
}

float complex fft_mat_get(fft_Matrix_cf* mat, size_t x, size_t y)
{
    size_t idx = y*mat->cols + x;
    FFT_ASSERT(idx<(mat->cols)*(mat->rows));
    return mat->data[idx];
}

void fft_mat_set(fft_Matrix_cf* mat, size_t x, size_t y, float complex e)
{
    size_t idx = y*mat->cols + x;
    FFT_ASSERT(idx<(mat->cols)*(mat->rows));
    mat->data[idx] = e;
}
"# }}</code></pre>
<div>{ r#"Great! now we have data structures we can use to create some memory leaks"# }</div>
<div>{ r#"or not that - let's implement a discrete fourier tranform using our vector to get a grip on what we're actually doing"# }</div>
<pre><code>{{ r#"
// [[ ALGORITHMS ]]

void dft(fft_Vec_cf* series, fft_Vec_cf* dft_series);
void dft_inverse(fft_Vec_cf* dft_series, fft_Vec_cf* series);
...

// -- Discrete Fourier Transform -- 

void dft(fft_Vec_cf* series, fft_Vec_cf* dft_series)
{
    for (size_t i=0; i<dft_series->size; ++i) {
        float complex res = 0.f;
        for (size_t j=0; j<series->size; ++j) {
            float complex xn = fft_vec_get(series, j);
            res += xn * cexp((-2.f*I*M_PI*(float)j*(float)i)/(float)dft_series->size);
        }
        fft_vec_set(dft_series, i, res);
    }
}

void dft_inverse(fft_Vec_cf* dft_series, fft_Vec_cf* series)
{
    for (size_t i=0; i<series->size; ++i) {
        float complex res = 0.f;
        for (size_t j=0; j<dft_series->size; ++j) {
            float complex xn = fft_vec_get(dft_series, j);
            res += xn * cexp((2.f*I*M_PI*(float)j*(float)i)/(float)series->size);
        }
        res /= series->size;
        fft_vec_set(series, i, res);
    }
}
"# }}</code></pre>
<div>{ r#"Cool - now we have a pretty good idea what we're doing"# }</div>
<div>{ r#"Basically calculating the center of mass of our function when wrapped around the origin for a given frequency, which maps to the index in our resulting vector"# }</div>
<div>{ r#"And for computing the inverse, we go the opposite way by using omega^-1 - aka the root of unity that we are evaluating at to the -1 power."# }</div>
<div>{ r#"Now we can write some tests to make sure we did everything right"# }</div>
<pre><code>{{ r#"
TEST(test_dft)
{
    size_t n = 8;
    fft_Vec_cf a = fft_vec_alloc(n);
    fft_Vec_cf b = fft_vec_alloc(n);
    fft_Vec_cf c = fft_vec_alloc(n);

    fft_vec_set(&a, 0,  1.f +  0.f * I);
    fft_vec_set(&a, 1,  2.f + -1.f * I);
    fft_vec_set(&a, 2,  0.f + -1.f * I);
    fft_vec_set(&a, 3, -1.f +  2.f * I);
    fft_vec_set(&a, 4,  1.f +  0.f * I);
    fft_vec_set(&a, 5,  2.f + -1.f * I);
    fft_vec_set(&a, 6,  0.f + -1.f * I);
    fft_vec_set(&a, 7, -1.f +  2.f * I);


    fft_vec_set(&c, 0,  4.f +  0.f * I);
    fft_vec_set(&c, 1,  0.f +  0.f * I);
    fft_vec_set(&c, 2, -4.f + -4.f * I);
    fft_vec_set(&c, 3,  0.f +  0.f * I);
    fft_vec_set(&c, 4,  0.f + -4.f * I);
    fft_vec_set(&c, 5,  0.f +  0.f * I);
    fft_vec_set(&c, 6,  8.f +  8.f * I);
    fft_vec_set(&c, 7,  0.f +  0.f * I);

    dft(&a, &b);
    fft_vec_free(&a);

    for (size_t k = 0; k < n; ++k) {
        ASSERT_CF_EQ(fft_vec_get(&b, k), fft_vec_get(&c, k))
    }

    fft_vec_free(&b);
    fft_vec_free(&c);
}

TEST(test_dft_inverse)
{
    size_t n = 8;
    fft_Vec_cf a = fft_vec_alloc(n);
    fft_Vec_cf b = fft_vec_alloc(n);
    fft_Vec_cf c = fft_vec_alloc(n);

    fft_vec_set(&a, 0,  1.f +  0.f * I);
    fft_vec_set(&a, 1,  2.f + -1.f * I);
    fft_vec_set(&a, 2,  0.f + -1.f * I);
    fft_vec_set(&a, 3, -1.f +  2.f * I);
    fft_vec_set(&a, 4,  1.f +  0.f * I);
    fft_vec_set(&a, 5,  2.f + -1.f * I);
    fft_vec_set(&a, 6,  0.f + -1.f * I);
    fft_vec_set(&a, 7, -1.f +  2.f * I);


    fft_vec_set(&c, 0,  4.f +  0.f * I);
    fft_vec_set(&c, 1,  0.f +  0.f * I);
    fft_vec_set(&c, 2, -4.f + -4.f * I);
    fft_vec_set(&c, 3,  0.f +  0.f * I);
    fft_vec_set(&c, 4,  0.f + -4.f * I);
    fft_vec_set(&c, 5,  0.f +  0.f * I);
    fft_vec_set(&c, 6,  8.f +  8.f * I);
    fft_vec_set(&c, 7,  0.f +  0.f * I);

    dft_inverse(&c, &b);
    fft_vec_free(&c);

    for (size_t k = 0; k < n; ++k) {
        ASSERT_CF_EQ(fft_vec_get(&a, k), fft_vec_get(&b, k))
    }
    fft_vec_free(&a);
    fft_vec_free(&b);
}
"# }}</code></pre>
<div>{ r#"Awesome - lets see if that works..."# }</div>
</div>
<img src={"/build/20241125_fourier_transforms/images/0_dft_test_results.png"}/>
<div>
<div>{ r#"Now lets implement the the Fast Fourier Transform and see how much faster it is compared to the naive discrete approach.."# }</div>
<div>{ r#"Looking at "# }<a href={ "https://en.wikipedia.org/wiki/Butterfly_diagram" }>{ "this wikipedia page" }
</a>{ r#" really clears up what I found to be the complicated part of implementing the algorithm"# }</div>
<div>{ r#"Essentially what it is doing to recombine the recursively solved sub-problems is two expressions on each pair, which generate elements at their respective indices:"# }</div>
<pre><code>{{ r#"
y0 = x0 + x1*w
y1 = x0 - x1*w
"# }}</code></pre>
<div>{ r#"where "# }<code>{ r#"x0"# }</code>{ r#" and "# }<code>{ r#"x1"# }</code>{ r#" are elements of the same index in the transformed sub problems, in this case "# }<code>{ r#"x0"# }</code>{ r#" comes from the even index sub-problem."# }</div>
<div>{ r#""# }<code>{ r#"w"# }</code>{ r#" in the above is our omega value, which is essentially just a root of unity - there is a lot more that goes into why these "# }<a href={ "" }>{ "twiddle factors" }
</a>{ r#" do what they do, but the jist is that evaluating polynomials at inputs selected such that they cancel eachother out drastically reduces the number of operations that must be performed, and these specific complex numbers allow us to do that :-)"# }</div>
<div>{ r#"With all that knowledge, we are ready to implement our recursive-cooley-tukey fast fourier transform..."# }</div>
<pre><code>{{ r#"
double complex fft_omega(size_t n, size_t k)
{
    return cexp((-2.f*I*M_PI*k)/n);
}

void fft(fft_Vec_cf* series, fft_Vec_cf* dft_series)
{
    FFT_ASSERT(series->size == dft_series->size);
    if (series->size == 1) {
        fft_vec_set(dft_series, 0, fft_vec_get(series, 0));
        return;
    }
    size_t series_count = series->size;
    FFT_ASSERT(series_count % 2 == 0);

    // split the series into even and off
    fft_Vec_cf evn = fft_vec_alloc(series_count/2);
    fft_Vec_cf odd = fft_vec_alloc(series_count/2);
    for (size_t k = 0; k < series_count/2; ++k) {
        fft_vec_set(&evn, k, fft_vec_get(series, k*2));
        fft_vec_set(&odd, k, fft_vec_get(series, (k*2)+1));
    }

    // get dft of each part
    fft_Vec_cf evn_dft = fft_vec_alloc(series_count/2);
    fft_Vec_cf odd_dft = fft_vec_alloc(series_count/2);

    fft(&evn, &evn_dft);
    fft(&odd, &odd_dft);

    fft_vec_free(&evn);
    fft_vec_free(&odd);

    // butterfly them together
    for (size_t k = 0; k < series_count/2; ++k) {
        double complex w = fft_omega(series_count, k);
        double complex x0 = fft_vec_get(&evn_dft, k);
        double complex x1 = fft_vec_get(&odd_dft, k);

        double complex y0 = x0 + x1*w;
        double complex y1 = x0 - x1*w;

        fft_vec_set(dft_series, k, y0);
        fft_vec_set(dft_series, k+series_count/2, y1);
    }

    fft_vec_free(&evn_dft);
    fft_vec_free(&odd_dft);
}
"# }}</code></pre>
<div>{ r#"Pro tip: In debugging and fixing my implementation I found that it was easier to start with a pseudo-recursive implementation, where the recursive calls above to "# }<code>{ r#"fft"# }</code>{ r#" are first calls to what we know to be a working "# }<code>{ r#"dft"# }</code>{ r#"."# }</div>
<div>{ r#"Then we can write some more tests to see if this thing actually works: "# }</div>
</div>
<img src={"/build/20241125_fourier_transforms/images/1_test_fft_dft.png"}/>
<div>
</div>
<img src={"/build/20241125_fourier_transforms/images/2_fft_dft_test_out.png"}/>
<div>
<div>{ r#"You might notice I am calling an fft_inverse function in the tests, this is pretty much the same exact algorithm, but but with a slight difference in the butterfly recombination:"# }</div>
<pre><code>{{ r#"
double complex w = cexp((2.f*I*M_PI*(int)k)/series_count);
double complex x0 = fft_vec_get(&evn_dft, k);
double complex x1 = fft_vec_get(&odd_dft, k);

double complex y0 = 0.5f*(x0 + x1*w);
double complex y1 = 0.5f*(x0 - x1*w);
"# }}</code></pre>
<div>{ r#"Now we can explore the differences in speed of our two algorithms"# }</div>
<div>{ r#"We can do so by splitting the test we wrote into two sections that we will turn on and off with a macro definition, and timing each one while increasing the input (shifting "# }<code>{ r#"n"# }</code>{ r#" until one runs too slow)"# }</div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241125_fourier_transforms/images/3_dft_vs_fft.webm"} type="video/webm"/>
</video><div>
<div>{ r#"The difference only gets more noticeable as n grows, but on only n = 4096, a difference of over two seconds to about 500 milliseconds is significant"# }</div>
<div>{ r#"It is worth noting at this point that this implementation can be improved - for example by performing it in place rather than mking a recursive call at all"# }</div>
<div>{ r#"Additionally, the original Cooley-Tukey algorithm, which is what we've implemented, is limited by the fact that it can only work on inputs that are a power of two in size"# }</div>
<div>{ r#"This can also be improved by using what is called a mixed radix"# }</div>
<div>{ r#"Above we split into two sub problems, but we could have also chosen to split into 3, 5, 7, 11 or some other number of groups. Importantly, this choice can be made on sub problems independent of other sub problems or parent problems, meaning it is possible to dynamically split the input based on its prime factorization."# }</div>
<div>{ r#"For the theoretical, general purpose uber algorithm, the keen eyed will observe that for increasingly large prime inputs, writing more radixes for the mixed radix solution is a losing battle - i.e. there will always be a larger prime input that the algorithm will not be able to handle. For this case, we have the dft - although very slow, it will solve the problem on prime inputs."# }</div>
<div>{ r#"How is this dealt with in the real world though? Well, I would choose an implementation based on domain. For example, if this is going to be used for image processing, it could accept images up to certain dimensions and then implement radixes for all primes up to that number - sounds like a lot of work, but primes get more sparse the higher you go, so how many could it really be?"# }</div>
<div>{ r#"As an aside, we can find out..."# }</div>
<div>{ r#"I haven't seen too many images larger than 3000x3000 - of course there are images that need to be much much larger than this, but for the sake of argument..."# }</div>
<div>{ r#"we can write a program to find some primes like this..."# }</div>
</div>
<img src={"/build/20241125_fourier_transforms/images/4_primes_finder.png"}/>
<div>
<div>{ r#"The temptation to start i at 0 or 1 is extremely strong, but it is important to remember that neither are prime - 1 only has one factor, and 0 has infinite factors, so although they are special little guys, this isn't about them"# }</div>
<div>{ r#"We start at 2 and test whether our previously found primes divide evenly into our current potential prime - something we only have to do up until it is half of our test number (because past that point we are testing what would be factors that would be multiplied by factors we already tested to produce the result - thus a redundant test)."# }</div>
<div>{ r#"Additionally, we can break out of the loop when we find a factor, since we only need one factor besides 1 and i to prove i is not prime"# }</div>
<div>{ r#"And that is how we get our 27 lines of glorious prime finding action, now to run it..."# }</div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241125_fourier_transforms/images/5_primes_run.webm"} type="video/webm"/>
</video><div>
<div>{ r#"And Melissa wants me to do 10_000, so here we go..."# }</div>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241125_fourier_transforms/images/6_stinky_request_primes_run.webm"} type="video/webm"/>
</video><div>
<div>{ r#"So if we want to accept images up to 10_000x10_000, we have to implement 1_229 radixes... Not a super realistic solution, so how does one generate these implementations? There must be a pattern that can be exploited, but for now I'm going to move on to a 2 dimensional implementation without writing any other radixes (to keep things simple for myself)."# }</div>
<div>{ r#"Looking at a bunch of different things, including "# }<a href={ "https://www.robots.ox.ac.uk/~az/lectures/ia/lect2.pdf" }>{ "this pdf" }
</a>{ r#" and "# }<a href={ "https://www.youtube.com/watch?v=v743U7gvLq0" }>{ "this video" }
</a>{ r#""# }</div>
<div>{ r#"it seems that a fourier transform in two dimensions can be performed via transforming all the rows individually as if they were independent 1 dimensional transforms"# }</div>
<div>{ r#"and then repeating the process on the columns formed by the transformed rows"# }</div>
<div>{ r#"Before implementing this algorithm on the  Matrix data type, I'll first do a slow version where I translate the matrices to arrays of our vector type and just call our vector functions on the row and column vectors..."# }</div>
<pre><code>{{ r#"
void fft_matrix(fft_Matrix_cf* mat)
{
    size_t img_rows = mat->rows;
    size_t img_cols = mat->cols;
    fft_Vec_cf rows[img_rows];
    fft_Vec_cf dft_rows[img_rows];
    fft_Vec_cf cols[img_cols];
    fft_Vec_cf dft_cols[img_cols];

    // put the matrix rows in the rows array of vectors
    for (size_t k = 0; k < img_rows; ++k) {
        rows[k] = fft_vec_alloc(img_cols);
        for (size_t j = 0; j < img_cols; ++j) {
            fft_vec_set(&rows[k], j, fft_mat_get(mat, k, j));
        }
    }

    // perform an fft on each row
    for (size_t k = 0; k < img_rows; ++k) {
        dft_rows[k] = fft_vec_alloc(img_cols);
        fft(&rows[k], &dft_rows[k]);
        fft_vec_free(&rows[k]);
    }

    for (size_t k = 0; k < mat->rows; ++k) {
        for (size_t j = 0; j < mat->cols; ++j) {
            // double complex f = fft_vec_get(&dft_rows[k], j);
            // printf("%f+%fi ", creal(f), cimag(f));
        }
        // printf("\n");
    }

    // copy the rows over to the cols array of vectors
    for (size_t j = 0; j < img_cols; j++) {
        cols[j] = fft_vec_alloc(img_rows);
        for (size_t k = 0; k < img_rows; ++k) {
            fft_vec_set(&cols[j], k, fft_vec_get(&dft_rows[k], j));
        }
    }

    // free all the dft_rows vectors
    for (size_t k = 0; k < img_rows; ++k) {
        fft_vec_free(&dft_rows[k]);
    }

    // perform an fft on each column 
    for (size_t k = 0; k < img_cols; ++k) {
        dft_cols[k] = fft_vec_alloc(img_rows);
        fft(&cols[k], &dft_cols[k]);
        fft_vec_free(&cols[k]);
    }

    // copy the cols back into the matrix
    for (size_t k = 0; k < img_cols; ++k) {
        for (size_t j = 0; j < img_rows; ++j) {
            fft_mat_set(mat, j, k, fft_vec_get(&dft_cols[k], j));
        }
        fft_vec_free(&dft_cols[k]);
    }
}
"# }}</code></pre>
<div>{ r#"And of course the inverse transform I'm doing the same thing but first the columns and calling fft_inverse on everything instead"# }</div>
<div>{ r#"Now we can get into transforming some images!"# }</div>
<div>{ r#"We'll do this with the help of the PPM format and some arrays of "# }<code>{ r#"uint32_t"# }</code>{ r#"'s:"# }</div>
<pre><code>{{ r#"
size_t img_rows = 512, img_cols = 512;
uint32_t* img = (uint32_t*)malloc(img_rows*img_cols*sizeof(uint32_t));

...

void save_img_as_ppm(uint32_t* img, size_t rows, size_t cols, const char *file_path)
{
    FILE* f = fopen(file_path, "wb");
    if (f == NULL) {
        fprintf(stderr, "ERROR: could not write into file %s", file_path);//: %s\n", file_path, strerror(errno));
        exit(1);
    }
    size_t img_w = cols;
    size_t img_h = rows;
    fprintf(f, "P6\n%ld %ld 255\n", img_w, img_h);
    for (size_t y=0; y<img_h; ++y) {
        for (size_t x=0; x<img_w; ++x) {
            uint32_t pixel = img[x+y*cols];
            uint8_t bytes[3] = {
                (pixel&0x0000FF)>>8*0,// R
                (pixel&0x00FF00)>>8*1,// G
                (pixel&0xFF0000)>>8*2 // B
            };
            fwrite(bytes, sizeof(bytes), 1, f);
            assert(!ferror(f));
        }
    }
    fclose(f);
}
"# }}</code></pre>
<div>{ r#"Now the world is kind of our oyster in terms of generating interesting looking things, but to try and keep things understandable, I'm going to be trying to keep everythign grayscale & keeping our values from 0-256 with these functions:"# }</div>
<pre><code>{{ r#"
// multiply by 255 and put the vale in each channel
uint32_t get_color(double complex f) {
    double re = creal(f);
    double im = cimag(f);
    double mag = sqrt(re*re+im*im);
    uint32_t res = 0x00000;
    uint32_t coef = 0xFF;
    res = res | (((uint32_t)(mag*coef)&0xFF)<<8*0);
    res = res | (((uint32_t)(mag*coef)&0xFF)<<8*1);
    res = res | (((uint32_t)(mag*coef)&0xFF)<<8*2);
    return res;
}

// grab the end (which should be repeated for each channel) and normalize 0-1
double complex map_color(uint32_t color) {
    uint32_t r = color & 0xFF; // 0-255
    return (double complex)r/255.f; // 0-1
}
"# }}</code></pre>
<div>{ r#"Now, we can set some pixels in our image, fourier transform it, inverse fourier transform it, and hope it comes out the other end the same as the original image as a sort of sanity test..."# }</div>
<pre><code>{{ r#"
int main()
{
    size_t img_rows = 512, img_cols = 512;
    uint32_t* img = (uint32_t*)malloc(img_rows*img_cols*sizeof(uint32_t));

    fill_image_white(img, img_cols, img_rows);
    draw_circle(img_cols/2, img_rows/2, 15, img, img_cols, img_rows);

    save_img_as_ppm(img, img_rows, img_cols, "original.ppm");

    fft_Matrix_cf mat = fft_mat_alloc(img_rows, img_cols);

    for (size_t x = 0; x < img_cols; ++x) {
        for (size_t y = 0; y < img_rows; ++y) {
            fft_mat_set(&mat, x, y, map_color(img[x+y*img_cols]));
        }
    }

    fft_matrix(&mat);
    for (size_t y = 0; y < img_rows; ++y) {
        for (size_t x = 0; x < img_cols; ++x) {
            double cf = fft_mat_get(&mat, x, y);
            img[x+y*img_cols] = get_color(cf);
        }
    }
    save_img_as_ppm(img, img_rows, img_cols, "fft.ppm");

    for (size_t y = 0; y < img_rows; ++y) {
        for (size_t x = 0; x < img_cols; ++x) {
            double cf = fft_mat_get(&mat, x, y);
            img[x+y*img_cols] = get_color(cf);
        }
    }

    fft_matrix_inverse(&mat);
    for (size_t x = 0; x < img_cols; ++x) {
        for (size_t y = 0; y < img_rows; ++y) {
            double cf = fft_mat_get(&mat, x, y);
            img[x+y*img_cols] = get_color(cf);
        }
    }
    save_img_as_ppm(img, img_rows, img_cols, "restored.ppm");

    fft_mat_free(&mat);
    free(img);
    return 0;
}
"# }}</code></pre>
</div><video autoplay=true width=500 loop=true>
<source src={"/build/20241125_fourier_transforms/images/7_original_restored.webm"} type="video/webm"/>
</video><div>
<div>{ r#"Awesome, we can also take a look at "# }</div>
<div>{ r#""# }</div>
</div></span>});
        Self {
            posts: post_map,
        }
    }
}