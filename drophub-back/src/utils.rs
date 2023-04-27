pub trait Inspect {
    type InspectType;

    fn inspect_fail<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self::InspectType);
}

impl<T> Inspect for Option<T> {
    type InspectType = ();

    fn inspect_fail<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self::InspectType),
    {
        if self.is_none() {
            f(&());
        }
        self
    }
}

impl<T, E> Inspect for Result<T, E> {
    type InspectType = E;

    fn inspect_fail<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self::InspectType),
    {
        if let Err(err) = &self {
            f(err);
        }
        self
    }
}
