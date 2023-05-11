use qrcode::{render::svg, QrCode as QrCodeLib};
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props<T>
where
    T: PartialEq,
{
    pub value: T,
    pub size: u32,
    #[prop_or(String::from("#000000"))]
    pub color: String,
    #[prop_or(String::from("#f8f9fa"))]
    pub bg_color: String,
}

#[function_component(QrCode)]
pub fn qrcode<T>(props: &Props<T>) -> Html
where
    T: PartialEq + AsRef<[u8]>,
{
    let qr_handle = use_state({
        let props = props.clone();
        move || {
            let code = QrCodeLib::new(&props.value).expect_throw("failed to generate QR code");
            let image = code
                .render::<svg::Color>()
                .min_dimensions(props.size, props.size)
                .max_dimensions(props.size, props.size)
                .light_color(svg::Color(&props.bg_color))
                .dark_color(svg::Color(&props.color))
                .quiet_zone(false)
                .build();

            Html::from_html_unchecked(image.into())
        }
    });

    html! {
        {(*qr_handle).clone()}
    }
}
