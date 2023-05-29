use std::fmt::Debug;

use wasm_bindgen::UnwrapThrowExt;

use crate::hooks::{NotifyManager, NotifyProps};

pub trait UnwrapNotifyExt<T>: Sized {
    fn unwrap_notify(self, man: &NotifyManager) -> T {
        self.expect_notify(man, "`expect_notify` failed")
    }

    fn expect_notify(self, man: &NotifyManager, msg: &str) -> T;
}

impl<T, E> UnwrapNotifyExt<T> for Result<T, E>
where
    E: Debug,
{
    fn unwrap_notify(self, man: &NotifyManager) -> T {
        match self {
            Ok(ok) => ok,
            Err(ref err) => {
                let msg = "Called `Result::unwrap()` on an `Err` value";
                man.show_notify(NotifyProps::error(format!("{msg}: {err:?}")));
                let _ = self.unwrap_throw();
                unreachable!()
            }
        }
    }

    fn expect_notify(self, man: &NotifyManager, msg: &str) -> T {
        match self {
            Ok(ok) => ok,
            Err(ref err) => {
                man.show_notify(NotifyProps::error(format!("{msg}: {err:?}")));
                let _ = self.expect_throw(msg);
                unreachable!()
            }
        }
    }
}

impl<T> UnwrapNotifyExt<T> for Option<T> {
    fn unwrap_notify(self, man: &NotifyManager) -> T {
        match self {
            Some(v) => v,
            None => {
                let msg = "Called `Option::unwrap()` on a `None` value";
                man.show_notify(NotifyProps::error(msg));
                let _ = self.expect_throw(msg);
                unreachable!()
            }
        }
    }

    fn expect_notify(self, man: &NotifyManager, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                man.show_notify(NotifyProps::error(msg));
                let _ = self.expect_throw(msg);
                unreachable!()
            }
        }
    }
}
