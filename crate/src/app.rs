use crate::list_page::ListPage;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/"]
    List,
    #[to = "/article/{id}"]
    Detail(u32),
}

fn render_route(switch: AppRoute) -> Html {
    match switch {
        _ => html! {<ListPage/>},
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section class="hero is-link is-bold">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">{ "Blog with Yew and microCMS" }</h1>
                        </div>
                    </div>
                </section>
                <section class="section">
                    <div class="container">
                        <Router<AppRoute, ()>
                            render = Router::render(|switch: AppRoute| render_route(switch))
                        />
                    </div>
                </section>
                <footer class="footer">
                    <div class="content has-text-centered">
                        <p>{ "©︎ 2020 Yosuke Onoue"}</p>
                    </div>
                </footer>
            </>
        }
    }
}
