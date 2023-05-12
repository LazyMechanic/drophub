use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
}

#[function_component(FileUpload)]
pub fn file_upload(props: &Props) -> Html {
    html! {
        <div class="col
                    d-flex
                    flex-column
                    align-items-center"
        >
            <button
                class="btn
                       btn-light
                       d-flex
                       border
                       rounded
                       justify-content-center
                       align-items-center
                       mb-1"
                style="height: 100px;
                       width: 100px;
                       border-style: dashed !important;"
                type="button"
                onclick={&props.onclick}
            >
                <i class="bi
                           bi-plus-lg
                           text-secondary
                           fs-1"
                ></i>
            </button>
        </div>
    }
}
