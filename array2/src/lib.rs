/// Elements contained must implement `Clone`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
width: usize,
height: usize,
}

impl Array2<T>{
    /*Creates an Array2 object with a width, height, and data */
pub fn new(width: usize, height: usize)->self;

/*This will create an Array2 where the Array2 is order ordered by rows */
pub fn from_row_major(width: usize, height: usize)->self;

/*This will create an Array2 where the Array2 is ordered by column */
pub fn from_column_major(width: usize, height: usize)->self;

/*This will return a value at a given position within Array2 */
pub fn iter_row_major(&self) -> &T;

/*This will return a value at a given position within Array2*/
pub fn iter_column_major(&self)->&T;
}
