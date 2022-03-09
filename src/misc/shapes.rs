use rand::{
    Rng,
    distributions::{
        Distribution,
        Standard
    }
};
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Shape {
    IBlock,
    LBlock,
    JBlock,
    SBlock,
    ZBlock,
    TBlock,
    OBlock,
}

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0..=6) {
            0 => Shape::IBlock,
            1 => Shape::LBlock,
            2 => Shape::JBlock,
            3 => Shape::SBlock,
            4 => Shape::ZBlock,
            5 => Shape::TBlock,
            _ => Shape::OBlock,
        }
    }
}