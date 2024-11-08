use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/posts")]
    Posts,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(About)]
fn about() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <>
            <h1>{ "About" }</h1>
            <p>{ "I am shane. I am a software engineer. This is a blog." }</p>
            <p>{ "Written in " } <a href={ "https://www.rust-lang.org" }> { "rust" } </a> { " with " } <a href={ "https://yew.rs" }> { "yew" } </a> { "." }</p>
            <button {onclick}>{ "Go Home" }</button>
        </>
    }
}

#[function_component(Posts)]
fn posts() -> Html {
    html! {
        <>
            <p1>{ "There are no posts yet... this will be one" }</p1>
            <div>
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            </div>
        </>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
            <h1>{ "Reshane" }</h1>
            <h3>{ "In Medias Res..." }</h3>
            <p>{ "Starting to document projects when I remember to... halfway through them" }</p>
            <div>
                <div>
                    <Link<Route> to={Route::Posts}>{ "Posts" }</Link<Route>>
                </div>
                <div>
                    <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                </div>
            </div>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::About => html! { <About/> },
        Route::Posts => html! { <Posts/> },
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
