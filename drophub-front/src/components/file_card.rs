use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FileCardProps {
    #[prop_or(FileKind::Unknown)]
    pub kind: FileKind,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
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
pub fn file_card(props: &FileCardProps) -> Html {
    html! {
        <div class="d-flex
                    flex-column
                    align-items-center
                    p-2"
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
                    <span class="placeholder col-10"></span>
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
                    ></span>
                } else {
                    {props.name.as_str()}
                }
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FileUploadProps {
    pub onclick: Callback<MouseEvent>,
}

#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    html! {
        <div class="d-flex
                    flex-column
                    align-items-center
                    p-2"
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
                <div
                    class="icon-link
                           text-secondary"
                >
                    <svg
                        class="bi"
                        role="img"
                        aria-label="Upload"
                        style="height: 1.25em;
                               width: 1.25em;"
                    >
                        <use href="#symbol-plus"/>
                    </svg>
                </div>
            </button>
        </div>
    }
}
