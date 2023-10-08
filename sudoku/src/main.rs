use std::env;
use csc411_image::{Read, GrayImage};
use array2::Array2;
use std::process::exit;

fn main() {
    assert!(env::args().len()<=2, "To many Arguements");
    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).expect("No image was provided");

    //Add pixels values to a vector
    let mut pixel_vector = Vec::new();
    for pixels in img.pixels {
        let pixel_value = pixels.value as u8; 
        if pixel_value > 9 || pixel_value == 0 {exit(1)};
        pixel_vector.push(pixel_value);
    }

    //Array2 - sorted by row & column.
    let _row_sorted = Array2::from_row_major(pixel_vector.clone(), 9, 9);
    let _column_sorted = Array2::from_column_major(pixel_vector.clone(), 9, 9);

    //Functions call
    sudoku_check(&_row_sorted);
    sudoku_check(&_column_sorted);
    sudoku_check_3x3(&_row_sorted);
 
    //Everything ok, return 0
    exit(0);

    
    
}
//Check rows and columns for sudoku
fn sudoku_check(arr: &Array2<u8>){
    let mut check_vec: Vec<u8> = Vec::new();
    for (i, el) in arr.iter_row_major().enumerate() {
        
        if check_vec.contains(&el){
            exit(1);
        } 
        check_vec.push(*el);
        if (i + 1) % 9 == 0 
        {
            check_vec = Vec::new();
        }
            
    }

}
//Check 3x3 squares for sudoku
fn sudoku_check_3x3(arr: &Array2<u8>){
    let mut check_vec: Vec<u8> = Vec::new();
    for i in 0..3 {
        for j in 0..3 {

            for x in 0..3 {
                for y in 0..3 {
                    let row = i * 3 + x;
                    let col = j * 3 + y;

                    if let Some(value) = arr.get(row, col) {
                        if check_vec.contains(&value){
                            exit(1);
                        }
                        check_vec.push(*value);
                    } else {
                        exit(1);
                    }
                }
            }
            check_vec = Vec::new();

        }
    }
}