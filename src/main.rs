use yew::prelude::*;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    html! {
        <p>{ "Hello, world!" }</p>
    }
}

fn main() {
    yew::start_app::<HelloWorld>();
}