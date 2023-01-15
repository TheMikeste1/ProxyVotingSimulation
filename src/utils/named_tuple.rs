pub struct NamedTuple<T> {
    pub name: String,
    pub value: T,
}

impl<T> NamedTuple<T> {
    pub fn new(name: String, value: T) -> NamedTuple<T> {
        NamedTuple { name, value }
    }
}
