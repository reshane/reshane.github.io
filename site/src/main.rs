use yew::prelude::*;
#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <head>{ "Reshane" }</head>
            <body>
                <h1>{ "In Medias Res..." }</h1>
                <p>{ "Starting to document projects when I remember to... halfway through them" }</p>
            </body>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
