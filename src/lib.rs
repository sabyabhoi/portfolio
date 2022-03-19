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
                   <li>{"HTML/CSS"}</li>
                   <li>{"Javascript/Typescript"}</li>
                   <ul>
                       <li>{"React.js"}</li>
                       <li>{"Next.js"}</li>
                   </ul>
                   <li>{"C/C++"}</li>
                   <li>{"Rust"}</li>
                   <li>{"Python"}</li>
                   <ul>
                       <li>{"Pandas"}</li>
                       <li>{"Matplotlib"}</li>
                       <li>{"Scikit-Learn"}</li>
                       <li>{"Tensorflow"}</li>
                   </ul>
                   <li>{"R"}</li>
                   <li>{"Bash scripting"}</li>
                   <li>{"LaTeX"}</li>
                   <li>{"groff/nroff"}</li>
                </ul>
            </Section>
            <Section title={"On the Web"}>
                <ul>
                    <li><Github/><a href="https://github.com/sabyabhoi/">{"@sabyabhoi"}</a></li>
                    <li><a href="https://www.linkedin.com/in/sabyabhoi/">{"sabyabhoi"}</a></li>
                    <li><a href="mailto:sabyabhoi@gmail.com">{"sabyabhoi@gmail.com"}</a></li>
                </ul>
            </Section>
        </div>
    }
}
