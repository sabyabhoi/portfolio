use yew::{function_component, html};

#[function_component(Intro)]
pub fn intro() -> Html {
    html! {
        <div class="intro">
            <h1>{"Hi, I'm Sabya."}</h1>
        </div>
    }
}
