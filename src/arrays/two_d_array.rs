pub fn two_d_array(){
    let mut array : [[i32;3];3] = [[1,2,3],[4,5,6],[7,8,9]];
    for element in array{
        for row in element{
            print!("{} ",row );
        }
        println!();
    }
    array[0][2] = 1;
    println!("{:?}",array[0]);

}