use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub title: String,
    pub children: Children,
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
