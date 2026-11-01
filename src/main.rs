use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Clone)]
struct TitleState(String);

#[derive(Clone)]
struct MainAppState {
    current_movie_input: Signal<String>,
    movie_list: Signal<Vec<String>>,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn Header() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        h1 { "{title.0}" }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        p { "" }
    }
}

#[component]
fn MainApp() -> Element {
    let add = move |_| {
        let app_state = use_context::<MainAppState>();
        let mut current_movie_list = app_state.movie_list.cloned();
        current_movie_list.push(app_state.current_movie_input.cloned());
        consume_context::<MainAppState>()
            .movie_list
            .set(current_movie_list.clone());
    };

    let update_input = move |evt: Event<FormData>| {
        consume_context::<MainAppState>()
            .current_movie_input
            .set(evt.value())
    };

    rsx! {
        div { id: "enter",
            input { onchange: update_input, type: "text" }
            button { onclick: add, id: "add-btn", "Add" }
            ul {
                {use_context::<MainAppState>().movie_list.cloned().iter().map(|s| rsx! {
                    li { "{s}" }
                })}
            }
        }
    }
}

#[component]
fn App() -> Element {
    let movie_list: Vec<String> = vec![];
    let movie_list = use_signal(|| movie_list.clone());

    let current_movie_input = use_signal(|| "".to_string());

    use_context_provider(|| TitleState("Popcorn Night!".to_string()));
    use_context_provider(|| MainAppState {
        current_movie_input,
        movie_list,
    });
    rsx! {
        document::Stylesheet { href: CSS }
        Header {}
        MainApp {}
        Footer {}
    }
}
