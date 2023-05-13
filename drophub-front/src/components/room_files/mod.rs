mod file_card;
mod file_upload;

use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::*;

use self::{
    file_card::{FileCard, FileKind},
    file_upload::FileUpload,
};
use crate::store::Store;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub placeholder: bool,
}

#[function_component(RoomFiles)]
pub fn room_files(props: &Props) -> Html {
    let store = use_store_value::<Store>();
    let room = &store.room;

    let files = room
        .info
        .files
        .iter()
        .map(|(file_id, file_meta)| {
            let onclick = if props.placeholder {
                Callback::noop()
            } else {
                // TODO: add onclick
                Callback::noop()
            };

            html! {
                <FileCard
                    placeholder={props.placeholder}
                    // TODO: determine file kind
                    kind={FileKind::Unknown}
                    name={file_meta.name.clone()}
                    {onclick}
                />
            }
        })
        .collect::<Html>();

    let upload = {
        let onclick = if props.placeholder {
            Callback::noop()
        } else {
            // TODO: add onclick
            Callback::noop()
        };

        html! { <FileUpload {onclick} /> }
    };

    html! {
        <div class="container-fluid
                    p-3"
        >
            <div class="row
                        row-cols-auto 
                        g-3"
            >
                {files}
                {upload}
            </div>
        </div>
    }
}
