use std::{cell::RefCell, fmt::Display, rc::Rc};

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob, RtcConfiguration, RtcDataChannel, RtcDataChannelEvent, RtcDataChannelInit,
    RtcPeerConnection, RtcPeerConnectionIceEvent, RtcSdpType, RtcSessionDescriptionInit,
};
use yew::platform::spawn_local;

pub trait NetworkManager {
    fn send_file(&self, file: &Blob) -> Result<(), JsValue>;
    fn send_text(&self, text: &str) -> Result<(), JsValue>;
}

const STUN_SERVER1: &str = "stun:stun.l.google.com:19302";
const STUN_SERVER2: &str = "stun:stun1.l.google.com:19302";

pub struct WebRtcServer {
    peer_conn: RtcPeerConnection,
    data_chan: RtcDataChannel,
    offer: RtcSessionDescriptionInit,
}

impl NetworkManager for WebRtcServer {
    fn send_file(&self, file: &Blob) -> Result<(), JsValue> {
        self.data_chan.send_with_blob(file)
    }

    fn send_text(&self, text: &str) -> Result<(), JsValue> {
        self.data_chan.send_with_str(text)
    }
}

impl WebRtcServer {
    pub async fn new() -> Result<Rc<RefCell<Self>>, JsValue> {
        let peer_conn = {
            let ice_servers = Array::new();
            {
                let server_entry = Object::new();

                Reflect::set(&server_entry, &"urls".into(), &STUN_SERVER1.into())?;
                Reflect::set(&server_entry, &"urls".into(), &STUN_SERVER2.into())?;

                ice_servers.push(&*server_entry);
            }

            let mut rtc_configuration = RtcConfiguration::new();
            rtc_configuration.ice_servers(&ice_servers);

            RtcPeerConnection::new_with_configuration(&rtc_configuration)?
        };

        let data_chan = {
            let mut data_channel_init = RtcDataChannelInit::new();
            data_channel_init.ordered(true);

            peer_conn
                .create_data_channel_with_data_channel_dict("entitiesChannel", &data_channel_init)
        };

        let this = Rc::new(RefCell::new(Self {
            peer_conn,
            data_chan,
            offer: RtcSessionDescriptionInit::new(RtcSdpType::Offer),
        }));

        let create_offer_closure = Closure::new({
            let this = this.clone();
            move |offer: JsValue| {
                let rtc_session_description: RtcSessionDescriptionInit =
                    offer.unchecked_into::<RtcSessionDescriptionInit>();

                let _promise = peer_conn
                    .set_local_description(&rtc_session_description)
                    .catch(&exception_handler("set_local_description failed"));

                this.borrow_mut().offer = rtc_session_description;
            }
        });

        JsFuture::from(
            peer_conn
                .create_offer()
                .then(&create_offer_closure)
                .catch(&exception_handler("create_offer failed")),
        )
        .await
        .expect_throw("create_offer future failed");

        create_offer_closure.forget();

        Ok(this)
    }
}

fn exception_handler<M>(msg: M) -> Closure<dyn FnMut(JsValue)>
where
    M: Display + 'static,
{
    Closure::new(move |value: JsValue| {
        // let notify_manager = get_notify();
        // notify_manager.show_notify(NotifyProps::error(format!("Exception handled: {msg}")));

        tracing::error!(?value, "Exception handled: {msg}");
    })
}
