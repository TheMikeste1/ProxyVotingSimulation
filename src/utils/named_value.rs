pub struct NamedValue<T> {
    pub name: String,
    pub value: T,
}

impl<T> NamedValue<T> {
    pub fn new(name: &str, value: T) -> NamedValue<T> {
        Self {
            name: name.into(),
            value,
        }
    }
}
