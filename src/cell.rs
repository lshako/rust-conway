#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Cell {
    Dead,
    Alive { color: CellColor },
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum CellColor {
    C1,
    C2,
}
