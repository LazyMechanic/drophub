use drophub::{EntityId, EntityKind, EntityMeta};
use yew::prelude::*;

use crate::components::Placeholder;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub id: EntityId,
    pub meta: EntityMeta,
}

fn icon(kind: EntityKind) -> Html {
    match kind {
        EntityKind::File => html! { <i class="bi bi-file-earmark"></i> },
        EntityKind::Text => html! { <i class="bi bi-text-left"></i> },
    }
}

#[function_component(EntityCard)]
pub fn entity_card(props: &Props) -> Html {
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
                {icon(props.meta.kind())}
            </button>
            <div
                class="text-truncate"
                style="max-width: 100px;"
            >
                <Placeholder<String>
                    enabled={props.loading}
                    content={props.meta.name().to_owned()}
                />
            </div>
        </div>
    }
}
