use yew::{function_component, html};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header>
            <div>{"Sabyasachi Bhoi"}</div>
            <button class="anim">{"dark"}</button>
        </header>
    }
}

