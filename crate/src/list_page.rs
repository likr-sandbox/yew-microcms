use crate::api::{fetch_articles, ArticleResponse};
use crate::article_list::ArticleList;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Response};

pub enum Msg {
    FetchReady(Result<ArticleResponse, Error>),
    Ignore,
}

pub struct ListPage {
    link: ComponentLink<Self>,
    fetching: bool,
    articles: Option<ArticleResponse>,
    task: Option<FetchTask>,
}

impl Component for ListPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetching: true,
            articles: None,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.articles = response.ok();
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <ArticleList articles={self.articles.clone().map(|a| a.contents).unwrap_or(vec![]) } />
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let callback = self.link.callback(
                move |response: Response<Json<Result<ArticleResponse, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchReady(data)
                    } else {
                        Msg::Ignore // FIXME: Handle this error accordingly.
                    }
                },
            );
            self.task = Some(fetch_articles(callback));
        }
    }
}
