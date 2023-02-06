pub struct NamedValue<T> {
    pub name: String,
    pub value: T,
}

impl<T> NamedValue<T> {
    pub fn new(name: String, value: T) -> NamedValue<T> {
        NamedValue { name, value }
    }
}
