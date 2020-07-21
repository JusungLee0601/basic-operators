pub use crate::types::changetype::ChangeType;
pub use crate::units::row::Row;

//Change, typing shows ChangeType, batch holds multiple potential changes
#[derive(Debug, Clone, PartialEq)]
pub struct Change {
    pub(crate) typing: ChangeType,
    pub(crate) batch: Vec<Row>
}

//Change functions
impl Change {
    //constructor
    pub fn new(typing: ChangeType, batch: Vec<Row>) -> Change {
        Change { typing, batch }
    }
}
