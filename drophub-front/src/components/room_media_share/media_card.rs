use yew::prelude::*;

use crate::components::Placeholder;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub kind: MediaKind,
    pub name: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MediaKind {
    File,
}

impl MediaKind {
    fn icon(&self) -> Html {
        match self {
            MediaKind::File => html! { <i class="bi bi-file-earmark"></i> },
        }
    }
}

#[function_component(MediaCard)]
pub fn media_card(props: &Props) -> Html {
    html! {
        <div class="d-flex
                    flex-column
                    align-items-center
                    gap-2"
        >
            <button
                class="btn
                       btn-shade-10
                       d-flex
                       border
                       border-0
                       rounded
                       justify-content-center
                       align-items-center"
                style="height: 100px;
                       width: 100px;"
                type="button"
            >
                {props.kind.icon()}
            </button>
            <div
                class="text-truncate"
                style="max-width: 100px;"
            >
                <Placeholder<String>
                    enabled={props.loading}
                    content={props.name.clone()}
                />
            </div>
        </div>
    }
}
