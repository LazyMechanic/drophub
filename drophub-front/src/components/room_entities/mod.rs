mod entity_announce;
mod entity_card;

use std::collections::HashMap;

use drophub::{Entity, EntityId};
use indexmap::IndexMap;
use yew::prelude::*;

use crate::components::room_entities::{entity_announce::EntityAnnounce, entity_card::EntityCard};

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub entities: IndexMap<EntityId, Entity>,
}

#[function_component(RoomEntities)]
pub fn room_entities(props: &Props) -> Html {
    let entities = props
        .entities
        .iter()
        .map(|(entity_id, entity_meta)| {
            html! {
                <EntityCard
                    loading={props.loading}
                    id={entity_id}
                    meta={entity_meta.clone()}
                />
            }
        })
        .collect::<Html>();

    let upload = html! {
        <EntityAnnounce />
    };

    html! {
        <div class="overflow-scroll-marker
                    overflow-scroll-marker-shade
                    border
                    border-0
                    rounded
                    w-100"
        >
            <div
                class="container-fluid
                       bg-shade
                       border
                       border-0
                       rounded
                       shadow
                       h-100
                       p-3
                       gap-2
                       overflow-y-auto"
            >
                <div class="row
                            row-cols-auto 
                            g-3"
                >
                    {entities}
                    {upload}
                </div>
            </div>
        </div>
    }
}
