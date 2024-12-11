fn main() {
    //an array of numbers 
    let numbers = [1,2,3,4,5];
    println!("Original array = {:?}",numbers);

    //creat a slice of 2nd and 3rd elemnt
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elemts slices - {:?}",slice1);

    //omit the start index 
    let slice2 = &numbers[..3];
    //This means the slice starts from index 0 and goes up to index 3(exculusive)
    println!("index 0 to index 3 sliceed = {:?}",slice2);

    //omit the end index 
    let slice3 = &numbers[2..];
    //This means the slice starts from index 2 and goes up to index 5(exculusive)
    println!("index 2 to index 5 sliceed = {:?}",slice3);


    //omit  the start index and the end index
    //reference the whole array
    let slice4  = &numbers[..];
    //This means the slice starts from index 0 and goes up to index 5 (exclusive)
    println!("index 0 to index 5 sliced = {:?}",slice4);
}
