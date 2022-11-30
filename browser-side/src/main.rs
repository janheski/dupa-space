use yew::prelude::*;

#[function_component]
fn App() -> Html {

    html! {
        <>
            <h1>{ "Dupa space" }</h1>
            <h2>{ "Przestrzeń na niepotrzebne rzeczy." }</h2>
            <p>{ "Tutaj możesz znaleźć wszystko to co przechowujesz w tej tajemnej skrytce." }</p>
            <p>{ "Użyj akcji \"W dupie to mam!\" do wsadzenia do dupy." }</p>
            <p>{ "W polu \"W dupie to masz!\" możesz wyszukać co masz w dupie." }</p>
            <p>{ "Użyj akcji \"Wysraj!\" aby usunąć rzecz z dupy." }</p>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
