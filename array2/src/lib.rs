
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
    pub fn iter_row_major(&self) -> impl Iterator<Item = &T>{
        self.data.iter().flatten()
    }

    /*This will return a value at a given position within Array2*/
    pub fn iter_column_major(&self)->impl Iterator<Item = &T>{
        (0..self.width).flat_map(move|col|(0..self.height).map(move|r| self.get(r,col).unwrap()))
    }

    pub fn get(&self, row: usize, column: usize)->Option<&T>{
        if row < self.width && column < self.height{
            Some(&self.data[row][column])
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
        let arr2 = Array2::from_column_major(vec,5,2);
        print!("{:?}",arr2.iter_row_major().collect::<Vec<_>>());
    }
}