use yew::{function_component, html, Properties};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header>
            <div>{"Sabyasachi Bhoi"}</div>
            <button class="anim">{"dark"}</button>
        </header>
    }
}

#[function_component(Intro)]
pub fn intro() -> Html {
    html! {
        <div class="intro">
            <h1>{"Hi, I'm Sabya."}</h1>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    title: String,
    content: String,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <div class="section">
            <h2 class="anim">{&props.title}</h2>
            <p>{&props.content}</p>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <Navbar/>
            <Intro/>
            <Section
            title={String::from("Work")}
            content={String::from("foo")}
            />
        </div>
    }
}
