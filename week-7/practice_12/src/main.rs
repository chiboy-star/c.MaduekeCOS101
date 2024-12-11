fn main() {
    //mutable array
    let mut colors =  ["red","green","yello","white"];

    println!("\nOriginal array = {:?}",colors);

    //mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("Frist slice = {:?}",sliced_colors);

    //change the vaule of the original slice at the first index 
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}",sliced_colors);
}
