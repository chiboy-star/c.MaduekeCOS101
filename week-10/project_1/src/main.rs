/*
Mr Ogbeifuna runs a series of electronics
shops at Alaba International Market in Lagos.
He has recently received a consignment of 30
laptop devices at the following cost; 10 HP
laptops at 650,000 each, 6 IBM laptops at
755,000 each, 10 Toshiba laptops at 550,000
each, and 4 Dell laptops at 850,000 each.
Using your knowledge of Rust struct and
method calculate the total cost supposing a
customer purchases 3 from each brand. 
*/

struct OgbeifunaElectronics {
    product_name: String,
    price_per_product: f64,
}

impl OgbeifunaElectronics {
    fn total_for_each_device(&self,quantity:u32) -> f64 {
        quantity as f64 * self.price_per_product
    }
}

fn main() {
    let hp = OgbeifunaElectronics {
        product_name: String::from("HP"),
        price_per_product: 650000.0,
    };
    let ibm = OgbeifunaElectronics {
        product_name: String::from("IBM"),
        
        price_per_product: 755000.0,
    };
    let toshiba = OgbeifunaElectronics {
        product_name: String::from("TOSHIBA"),
       
        price_per_product: 550000.0,
    };
    let dell = OgbeifunaElectronics {
        product_name: String::from("DELL"),
        
        price_per_product: 850000.0,
    };

    let quantity = 3;

    let grand_total = hp.total_for_each_device(quantity)
        + ibm.total_for_each_device(quantity)
        + toshiba.total_for_each_device(quantity)
        + dell.total_for_each_device(quantity);

    println!(
        "This customer purchased 3 of each product which came out to:
        {} for the {} laptop,
        {} for the {} laptop,
        {} for the {} laptop,
        {} for the {} laptop,
        which comes out to a grand total of = {}",
        hp.total_for_each_device(quantity),
        hp.product_name,
        ibm.total_for_each_device(quantity),
        ibm.product_name,
        toshiba.total_for_each_device(quantity),
        toshiba.product_name,
        dell.total_for_each_device(quantity),
        dell.product_name,
        grand_total
    );
}
