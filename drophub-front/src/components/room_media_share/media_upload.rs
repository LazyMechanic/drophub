use yew::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {}

#[function_component(MediaUpload)]
pub fn media_upload(_props: &Props) -> Html {
    html! {
        <div class="col
                    d-flex
                    flex-column
                    align-items-center"
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
                <i class="bi
                          bi-cloud-arrow-up"
                ></i>
            </button>
        </div>
    }
}
