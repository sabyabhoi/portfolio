use yew::{function_component, html};

mod components;
use components::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <Navbar/>
            <Intro/>
            <Section title={"Work"}>
                {"I'm a second-year engineering student at Birla Institute of Technology and Science. I'm a Linux enthusiast. I use GNU/Linux as my primary driver, and like to tinker around with new, free and open-source projects."}
            </Section>
            <Section title={"Skills"}>
                <ul>
                    <li>{"What is love?"}</li>
                </ul>
            </Section>
            <Section title={"On the Web"}>
                <ul>
                    <li>{"@sabyabhoi"}</li>
                    <li>{"sabyabhoi"}</li>
                    <li>{"sabyabhoi@gmail.com"}</li>
                </ul>
            </Section>
        </div>
    }
}
