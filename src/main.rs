use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::{
    header::Header,
};

mod pages;
use pages::{
    ask::Ask,
    jobs::Jobs,
    new::New,
    page_not_found::PageNotFound,
    show::Show,
    top::Top,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Top,
    #[at("/new")]
    New,
    #[at("/show")]
    Show,
    #[at("/ask")]
    Ask,
    #[at("/jobs")]
    Jobs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Header />

                <main class="container">
                    <Switch<Route> render={Switch::render(switch)} />
                </main>

                <footer class="container">
                    <small>
                        { "Built with " }<a href="https://www.rust-lang.org/">{ "Rust" }</a> 
                        { " and " }<a href="https://yew.rs/">{ "Yew" }</a>
                        { " • Styles by "}<a href="https://picocss.com/">{ "Pico" }</a>
                        { " • " }<a href="https://github.com/thehouseplant/hewn">{ "Source" }</a>
                    </small>
                </footer>
            </BrowserRouter>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Ask => {
            html! { <Ask /> }
        }
        Route::Jobs => {
            html! { <Jobs /> }
        }
        Route::New => {
            html! { <New /> }
        }
        Route::Show => {
            html! { <Show /> }
        }
        Route::Top => {
            html! { <Top /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}