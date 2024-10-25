fn main(){
    let p:f64 = 510000.00;
    let r:f64 = 5.0;
    // compound interest for depreciation
    let a1:f64 = p*(1.0-(r/100.00));
    println!("The depreciation for the first year is {}",a1);
    let a2:f64 = a1*(1.0-(r/100.00));
    println!("The depreciation for the first year is {}",a2);
    let a3:f64 = a2*(1.0-(r/100.00));
    println!("The depreciation for the first year is {}",a3);
}