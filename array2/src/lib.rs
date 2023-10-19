
/// Elements contained must implement `Clone`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
width: usize,
height: usize,
data: Vec<Vec<T>>,
}

impl <T: Clone> Array2<T>{
    /*Creates an Array2 object with a width, height, and data */
    pub fn new(value: T, width: usize, height: usize)->Self{
        //data Vec with default values
        let data = vec![vec![value; width]; height];

        Array2{
            data: data,
            width: width,
            height: height,
        }
    }

    /*This will create an Array2 where the Array2 is order ordered by rows */
    pub fn from_row_major(user_data: Vec<T>, width: usize, height: usize)->Self{
        if user_data.len() != width * height {
            panic!("Data length not equal to width * height");
        }
        else{
            let mut arr2_data = Vec::new();
            let mut counter = 0;
            let mut temp_vec = Vec::new();
            for _row in 0..height{
                for _column in 0..width{
                    /*The invariant here is that each element has been placed in the correct
                    position based on the current row and column */
                    temp_vec.push(user_data[counter].clone());
                    counter += 1;
                }
                arr2_data.push(temp_vec.clone());
                temp_vec.clear();
            }

            Array2{
                data: arr2_data,
                width: width,
                height: height,
            }
        }
    }

    /*This will create an Array2 where the Array2 is ordered by column */
    pub fn from_column_major(data: Vec<T>, width: usize, height: usize)->Self{
        if data.len() != width * height {
            panic!("Data length not equal to width * height");
        }
        else{
            let mut arr2_data = Vec::new();
            let mut temp_vec = Vec::new();
            let mut counter = 0;
            for _row in 0..width{
                for column in 0..height{
                    /*The invariant here is that every item has been placed in the correct position
                    within the 2d array based on the current row and column*/
                    if counter < height {
                        temp_vec.push(data[counter].clone());
                        arr2_data.push(temp_vec.clone());
                        temp_vec.clear();
                    }
                    else{
                        arr2_data[column].push(data[counter].clone());
                        temp_vec.clear();
                    }
                    counter += 1;
                }
            }

            Array2{
                data: arr2_data,
                width: width,
                height: height,
            }
        }
    }

    /*This will return a value at a given position within Array2 */
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize,usize,&T)>{
        /*The invariant here is that the 2d array is already organized by 
        row major format */
        (0..(self.height)).flat_map(move|row|(0..(self.width)).map(move|col| (row,col,self.get(row,col).unwrap())))
    }

    /*This will return a value at a given position within Array2*/
    pub fn iter_column_major(&self)->impl Iterator<Item = (usize,usize,&T)>{
        /*The invariant here is that every item read thus far lie in the same column, but
        different rows */
        /*This bit of code is code altered from lecture 9 of class */
        (0..(self.width)).flat_map(move|col|(0..(self.height)).map(move|r| (r,col,self.get(r,col).unwrap())))
    }

    pub fn get(&self, row: usize, column: usize)->Option<&T>{
        /* the invariant here some type of item will be returned. either a value from the
        the 2d array or None*/ 
        /*This bit of code is from lecture 9 of class */
        if row < self.height && column < self.width{
            Some(&self.data[row][column])
        }
        else{
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, column: usize)->Option<&mut T>{
        /* the invariant here some type of item will be returned. either a value from the
        the 2d array or None*/ 
        /*This bit of code is from lecture 9 of class */
        if row < self.height && column < self.width{
            Some(&mut self.data[row][column])
        }
        else{
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new(){
        let vec = Vec::from([1,2,3,4,5,6,7,8,9,0]);
        let mut arr2 = Array2::from_column_major(vec,5,2);
        print!("{:?}",arr2.data);
        print!("{:?}",arr2.get(0,1).unwrap());
        println!("Num columns: {:?}  num rows: {:?}", arr2.width, arr2.height);
        println!("{:?}",arr2.iter_row_major().collect::<Vec<_>>());
        print!("{:?}",arr2.iter_column_major().collect::<Vec<_>>());
        let i = arr2.get_mut(1, 2).unwrap();
        print!("{i}");
        *i = 100;
        print!("{i}");

    }
}