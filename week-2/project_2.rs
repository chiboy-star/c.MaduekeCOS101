fn main(){
    let t:f64 = 450000.00;//Toshiba
    let m:f64 = 1500000.00;//Mac
    let h:f64 = 750000.00;//HP
    let d:f64 = 2850000.00;//Dell
    let a:f64 = 250000.00;//Acer
    //sum
    let s = (2.0*t)+(1.0*m)+(3.0*h)+(3.0*d)+(1.0*a);
    println!("The sum is Equal to {}",s);
    //total Qty
    let q:f64 = 5.0;
    //average
    let av = s/q;
    println!("The average is equal to {}",av);
}