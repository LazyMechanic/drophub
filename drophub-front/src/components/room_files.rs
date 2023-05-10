use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::file_card::{FileCard, FileKind, FileUpload},
    store::Store,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub placeholder: bool,
}

#[function_component(RoomFiles)]
pub fn room_files(props: &Props) -> Html {
    let store = use_store_value::<Store>();

    let files = if props.placeholder {
        std::iter::repeat_with(|| {
            html! { <FileCard placeholder={true} /> }
        })
        .take(5)
        .collect::<Html>()
    } else {
        let room = store.room.as_ref().expect_throw("room not found");
        room.info
            .files
            .iter()
            .map(|(file_id, file_meta)| {
                // TODO: add onclick
                // TODO: determine file kind
                html! {
                    <FileCard
                        kind={FileKind::Unknown}
                        name={file_meta.name.clone()}
                        onclick={Callback::noop()}
                    />
                }
            })
            .collect::<Html>()
    };

    let upload = if props.placeholder {
        // TODO: add onclick
        html! { <FileUpload onclick={Callback::noop()} /> }
    } else {
        html! { <FileUpload onclick={Callback::noop()} /> }
    };

    html! {
        <div class="container-fluid
                    p-2"
        >
            <div class="row
                        row-cols-auto 
                        g-2"
            >
                {files}
                {upload}
            </div>
        </div>
    }
}
