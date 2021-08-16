pub enum Type {
    Int(i64),
    Float(f64),
    ReishiString(Vec<u8>),
    List(Vec<usize>),
}