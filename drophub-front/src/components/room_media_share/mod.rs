mod media_card;
mod media_upload;

use std::collections::HashMap;

use drophub::{FileId, FileMeta};
use yew::prelude::*;

use crate::components::room_media_share::{
    media_card::{MediaCard, MediaKind},
    media_upload::MediaUpload,
};

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub medias: HashMap<FileId, FileMeta>,
}

#[function_component(RoomMediaShare)]
pub fn room_media_share(props: &Props) -> Html {
    let medias = props
        .medias
        .iter()
        .map(|(file_id, file_meta)| {
            html! {
                <MediaCard
                    loading={props.loading}
                    kind={MediaKind::File}
                    name={file_meta.name.clone()}
                />
            }
        })
        .collect::<Html>();

    let upload = html! {
        <MediaUpload />
    };

    html! {
        <div class="overflow-scroll-marker
                    overflow-scroll-marker-shade
                    border
                    border-0
                    rounded"
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
                    {medias}
                    {upload}
                </div>
            </div>
        </div>
    }
}
