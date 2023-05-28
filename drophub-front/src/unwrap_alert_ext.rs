use std::fmt::Debug;

use wasm_bindgen::UnwrapThrowExt;

use crate::hooks::{AlertManager, AlertProps};

pub trait UnwrapAlertExt<T>: Sized {
    fn unwrap_alert(self, man: &AlertManager) -> T {
        self.expect_alert(man, "`expect_alert` failed")
    }

    fn expect_alert(self, man: &AlertManager, msg: &str) -> T;
}

impl<T, E> UnwrapAlertExt<T> for Result<T, E>
where
    E: Debug,
{
    fn unwrap_alert(self, man: &AlertManager) -> T {
        match self {
            Ok(ok) => ok,
            Err(ref err) => {
                let msg = "Called `Result::unwrap()` on an `Err` value";
                man.show_alert(AlertProps::error(format!("{msg}: {err:?}")));
                let _ = self.unwrap_throw();
                unreachable!()
            }
        }
    }

    fn expect_alert(self, man: &AlertManager, msg: &str) -> T {
        match self {
            Ok(ok) => ok,
            Err(ref err) => {
                man.show_alert(AlertProps::error(format!("{msg}: {err:?}")));
                let _ = self.expect_throw(msg);
                unreachable!()
            }
        }
    }
}

impl<T> UnwrapAlertExt<T> for Option<T> {
    fn unwrap_alert(self, man: &AlertManager) -> T {
        match self {
            Some(v) => v,
            None => {
                let msg = "Called `Option::unwrap()` on a `None` value";
                man.show_alert(AlertProps::error(msg));
                let _ = self.expect_throw(msg);
                unreachable!()
            }
        }
    }

    fn expect_alert(self, man: &AlertManager, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                man.show_alert(AlertProps::error(msg));
                let _ = self.expect_throw(msg);
                unreachable!()
            }
        }
    }
}
