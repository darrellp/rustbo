#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Card {
    Ranked(u8),
    Wild
}
