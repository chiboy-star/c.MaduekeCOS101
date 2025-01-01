/*
Nigerian Breweries Plc, the pioneer and largest brewing Company in Nigeria
was incorporated in 1946 as "Nigerian Brewery Limited". Their rich portfolio of
high-quality Lager, Stout, Non-alcoholics and Spirit are uniquely outstanding
which is why they are Nigeria’s number one choice, as shown below.

Lager         Stout        Non-Alcoholic
33 Export     Legend       Maltina
Desperados    Turbo King    Amstel Malta
Goldberg       Williams    Malta Gold
Gulder         Fayrouz
Heineken
Star

You have been hired to develop a Rust application to create a file that saves
the high-quality categories of drinks, as indicated in the table.
*/

use std::io::Write;

fn main(){
    let mut file = std::fs::File::create("Nigerian Breweries Database.txt").expect("create failed");
    file.write_all("    Lager   | Stout         | Non-Alcoholic\n
    33 Export   | Legend    | Maltina\n
    Desperados  | Turbo King| Amstel Malta\n
    Goldberg    | Williams  | Malta Gold\n
    Gulder      |           | Fayrouz\n
    Heineken    |           |        \n
    Star        |           |       \n".as_bytes()).expect("write failed");
}