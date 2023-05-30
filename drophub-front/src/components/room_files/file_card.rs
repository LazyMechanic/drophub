use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(FileKind::Unknown)]
    pub kind: FileKind,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub placeholder: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Img,
    Txt,
    Doc,
    Audio,
    Video,
    Unknown,
    // TODO: fill
}

impl FileKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileKind::Img => "img",
            FileKind::Txt => "txt",
            FileKind::Doc => "doc",
            FileKind::Audio => "audio",
            FileKind::Video => "video",
            FileKind::Unknown => "unknown",
        }
    }
}

// TODO: Make files clickable

#[function_component(FileCard)]
pub fn file_card(props: &Props) -> Html {
    html! {
        <div class="d-flex
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
                       width: 100px;"
                type="button"
                onclick={&props.onclick}
            >
                if props.placeholder {
                    <span class="placeholder col-10">
                        {"kind"}
                    </span>
                } else {
                    {props.kind.as_str()}
                }
            </button>
            <div
                class="text-truncate"
                style="max-width: 100px;"
            >
                if props.placeholder {
                    <span
                        class="placeholder col-12"
                        style="width: 100px;"
                    >{"file.txt"}</span>
                } else {
                    {props.name.as_str()}
                }
            </div>
        </div>
    }
}
