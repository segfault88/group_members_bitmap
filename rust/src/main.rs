use std::fs::File;
use roaring::bitmap::RoaringBitmap;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut bm = RoaringBitmap::new();
    
    let mut count: u32 = 0;
    let mut skipped: u32 = 0;

    match File::open("../group_members.csv") {
        Ok(file) => {
            println!("file opened");

            let mut reader = csv::Reader::from_reader(file);
            for result in reader.records() {
                count+=1;

                match result {
                    Ok(record) => {
                        if record.len() != 2 {
                            eprintln!("invalid record count: {} value: {:?}", count, record);
                            continue;
                        }

                        let group_id_str = &record[0];
                        let member_id_str = &record[1];

                        let group_id: u32 = match group_id_str.parse::<u32>() {
                            Ok(group_id) => group_id,
                            Err(err) => {
                                skipped+=1;
                                _=err;
                                // eprintln!("invalid group_id: {} count: {}", err, count);
                                continue;
                            }
                        };

                        let member_id: u32 = match member_id_str.parse::<u32>() {
                            Ok(member_id) => member_id,
                            Err(err) => {
                                skipped+=1;
                                _=err;
                                // eprintln!("invalid member_id: {} count: {}", err, count);
                                continue;
                            }
                        };

                        _ = group_id;
                        _ = bm.insert(member_id);

                        // println!("group_id: {}, member_id: {}", group_id, member_id);
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

    println!("done in: {:?} skipped: {}, len: {:?}", Instant::now().duration_since(start),  skipped, bm.len());
}
