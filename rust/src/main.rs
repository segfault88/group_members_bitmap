use std::fs::File;

fn main() {
    match File::open("../group_members.csv") {
        Ok(file) => {
            let mut reader = csv::Reader::from_reader(file);
            for result in reader.records() {
                match result {
                    Ok(record) => {
                        if record.len() != 2 {
                            eprintln!("Invalid record: {:?}", record);
                            continue;
                        }
                        let name = &record[0];
                        let email = &record[1];
                        println!("Name: {}, Email: {}", name, email);
                        return;
                    }
                    Err(err) => {
                        eprintln!("Error reading record: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
}
