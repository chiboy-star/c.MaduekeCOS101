/*
The Federal Government of Nigeria has tasked the EFCC to produce the files of
convicted Ministers from different geopolitical zones in the country. However, due to
a recent outbreak at the Information Service Department of the Abuja Headquarters,
the hard copy files were lost. Nevertheless, the cloud backup system dataset are
safe, but they exist in separate datasets.
As an expert developer, the EFCC has consulted you to develop a rust program that
would merge these separate datasets into one single output. Should you choose to
accept the task, implement the program with your knowledge of arrays and vectors.
*/

fn main() {
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    for i in 0..commissioners.len() {
        println!(
            "{:>3} | {:<35} | {:<17} | {}",
            i + 1,
            commissioners[i],
            zones[i],
            ministries[i]
        );
    }
}

