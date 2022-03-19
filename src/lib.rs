use yew::{function_component, html, Children, Properties};

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
    children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <div class="section">
            <h2 class="anim">{&props.title}</h2>
            <p>{for props.children.iter()}</p>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <Navbar/>
            <Intro/>
            <Section title={String::from("Work")}>
                {"I'm a second-year engineering student at Birla Institute of Technology and Science. I'm a Linux enthusiast. I use GNU/Linux as my primary driver, and like to tinker around with new, free and open-source projects."}
            </Section>
        </div>
    }
}
