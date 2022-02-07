use app::csv::{read_file, write_file};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

fn main() {
    match read_file::<Record>("data/tests/read.csv") {
        Ok(records) => {
            for record in records {
                println!("{:?}", record);
            }
        }
        Err(_) => panic!("Could not read data/tests/read.csv"),
    };

    let records = vec![
        Record {
            city: String::from("Paris"),
            region: String::from("Paris"),
            country: String::from("France"),
            population: Some(10_000_000),
        },
        Record {
            city: String::from("Barcelone"),
            region: String::from("Catalogne"),
            country: String::from("Espagne"),
            population: Some(3_000_000),
        },
    ];

    match write_file("data/tests/write.csv", records) {
        Ok(_) => println!("done! see ./data/tests/write.csv"),
        Err(_) => println!("Something goes wrong"),
    };
}
