use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod generated;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/blog")]
    Blog,
    #[at("/posts/:id")]
    Post { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(About)]
fn about() -> Html {
    html! {
        <>
            <div>
                <div>
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                </div>
                <div>
                    <Link<Route> to={Route::Blog}>{ "Blog" }</Link<Route>>
                </div>
            </div>
            <h1>{ "About" }</h1>
            <p>{ "I am shane. I am a software engineer. This is a blog." }</p>
            <p>{ "Written in " } <a href={ "https://www.rust-lang.org" }> { "rust" } </a> { " with " } <a href={ "https://yew.rs" }> { "yew" } </a> { "." }</p>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct PostProps {
    id: String,
}

#[function_component(Post)]
fn posts(props: &PostProps) -> Html {
    let posts = generated::posts::Posts::new();
    match posts.posts.get(&props.id) {
        Some(post) => {
            html!{ 
                <>
                    <div>
                        <div>
                            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                        </div>
                        <div>
                            <Link<Route> to={Route::Blog}>{ "Blog" }</Link<Route>>
                        </div>
                        <div>
                            <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                        </div>
                    </div>
                    { post.clone() }
                    <div>{ "why would you actually read all of that?" }</div>
                    <div>
                        <div>
                            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                        </div>
                        <div>
                            <Link<Route> to={Route::Blog}>{ "Blog" }</Link<Route>>
                        </div>
                        <div>
                            <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                        </div>
                    </div>
                </>
            }
        },
        None => html! { <Redirect<Route> to={Route::Blog}/> },
    }
}

#[function_component(Blog)]
fn blog() -> Html {
    let posts = generated::posts::Posts::new();

    let mut posts_names: Vec<&String> = posts.posts.keys().collect::<Vec<&String>>();
    posts_names.sort_by(|a, b| b.cmp(a));

    html! {
        <>
            <div>
                <div>
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                </div>
                <div>
                    <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                </div>
            </div>
            <h1>{ "Posts" }</h1>
            {
                for posts_names.iter().map(|name| html_nested! { <div><Link<Route> to={Route::Post { id: name.to_string() }}>{ format!("{name}") }</Link<Route>></div>} )
            }
        </>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
            <div>
                <div>
                    <Link<Route> to={Route::Blog}>{ "Blog" }</Link<Route>>
                </div>
                <div>
                    <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                </div>
            </div>
            <h1>{ "Reshane" }</h1>
            <p>{ "Welcome to the blog. I write down things I'm learning here" }</p>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::About => html! { <About/> },
        Route::Blog => html! { <Blog/> },
        Route::Post { id } => html! { <Post {id} /> },
        Route::NotFound => html! { <h1>{ "404 Not Found :(" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
