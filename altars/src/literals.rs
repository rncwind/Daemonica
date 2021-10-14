// Conventionally these things are stored as void*s or Objects. (un)fortunatley
// rust takes inspiration from ML so all of our types need to be concrete.
// To handle the fact we have a lot of different literals that can exist we
// wrap them inside a union as variants. This gives us the illusion that we are
// only refering to some Literal type, when it contains some value.
// As a wise man once said, there is power in a union.

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    StrLit(String),
    Empty,
}
