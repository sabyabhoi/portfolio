use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct IconElementProps {
    pub link: String,
    pub children: Children,
}

#[function_component(IconElement)]
pub fn icon_element(props: &IconElementProps) -> Html {
    html! {
        <li>
            {for props.children.iter()}
            {&props.link}
        </li>
    }
}
