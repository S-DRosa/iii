/// Elements contained must implement `Clone`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
width: usize,
height: usize,
}
/*Creates an Array2 object with a width, height, and data */
pub fn new(width: usize, height: usize)->self;
/*This will return an iterator so the Array2 can be iterated through by row*/
pub fn iter_row_major(&self) -> iterator<T>;

/*This will return an iterator so the Array2 can be iterated through by column*/
pub fn iter_column_major(&self)->iterator<T>;

/*This will return an iterator so the Array2 can be iterated through by 3x3 blocks*/
pub fn iter_square(&self)->iterator<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
