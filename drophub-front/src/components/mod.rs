pub mod connect_room_form;
pub mod copy_input;
pub mod create_room_form;
pub mod footer;
pub mod full_screen_loading;
pub mod full_screen_notify;
pub mod header;
pub mod notify;
pub mod placeholder;
pub mod qr_code;
//pub mod room_control;
pub mod room_control2;
//pub mod room_files;
pub mod room_media_share;

pub use self::{
    connect_room_form::ConnectRoomForm,
    copy_input::CopyInput,
    create_room_form::CreateRoomForm,
    footer::Footer,
    full_screen_loading::FullScreenLoading,
    full_screen_notify::FullScreenNotify,
    header::Header,
    notify::{NotifyContainer, NotifyKind},
    placeholder::Placeholder,
    qr_code::QrCode,
    room_control2::RoomControl,
    room_media_share::RoomMediaShare,
};
