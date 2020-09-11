use crate::api::Article;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

fn render_article(article: &Article) -> Html {
    let content = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    content.set_inner_html(&article.content);
    html! {
        <div class="box">
            <h2 class="title is-2">{ &article.title }</h2>
            <div class="content">{VNode::VRef(Node::from(content))}</div>
        </div>
    }
}

pub struct ArticleList {
    pub articles: Vec<Article>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub articles: Vec<Article>,
}

impl Component for ArticleList {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            articles: props.articles,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.articles = props.articles;
        true
    }

    fn view(&self) -> Html {
        let articles = self
            .articles
            .iter()
            .map(|article| render_article(&article))
            .collect::<Html>();
        html! {
            <div id="container">
                { articles }
            </div>
        }
    }
}
