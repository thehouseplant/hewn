use std::fmt::Display;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq,
{
    data: T,
}

#[function_component(PollPart)]
pub fn poll_part<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Display,
{
    html! {
        <p>
            { &props.data }
        </p>
    }
}