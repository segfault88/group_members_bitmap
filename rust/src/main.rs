use humanize_bytes::humanize_bytes_binary;
use roaring::bitmap::RoaringBitmap;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use thousands::Separable;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let mut bitmaps: HashMap<u32, RoaringBitmap> = HashMap::new();

    let mut count: u32 = 0;
    let mut skipped: u32 = 0;

    let file = File::open("../group_members.csv")?;
    println!("input file opened");

    let mut reader = csv::Reader::from_reader(file);
    for result in reader.records() {
        count += 1;
        if count % 1000000 == 0 {
            println!("count: {}", count.separate_with_commas());
        }

        let record = result?;
        if record.len() != 2 {
            eprintln!("invalid record count: {} value: {:?}", count, record);
            continue;
        }

        let group_id_str = &record[0];
        let member_id_str = &record[1];

        let group_id: u32 = match group_id_str.parse() {
            Ok(group_id) => group_id,
            Err(err) => {
                skipped += 1;
                _ = err;
                // eprintln!("invalid group_id: {} count: {}", err, count);
                continue;
            }
        };

        let member_id: u32 = match member_id_str.parse() {
            Ok(member_id) => member_id,
            Err(err) => {
                skipped += 1;
                _ = err;
                // eprintln!("invalid member_id: {} count: {}", err, count);
                continue;
            }
        };

        let bitmap = bitmaps.entry(group_id).or_insert(RoaringBitmap::new());
        bitmap.insert(member_id);
    }

    println!("bitmaps: {}", bitmaps.len());
    let mut total_bytes = 0;

    for (group_id, bitmap) in bitmaps.iter() {
        let mut bytes = vec![];
        bitmap.serialize_into(&mut bytes)?;

        total_bytes += bytes.len();

        println!(
            "group_id: {}, len: {:?}, size: {}",
            group_id,
            bitmap.len(),
            humanize_bytes_binary!(bytes.len())
        );

        let path = format!("group_{}.roaring", group_id);

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path);

        match f {
            Ok(mut file) => {
                file.write(bytes.as_slice())?;
            }
            Err(err) => {
                eprintln!("error opening output file: {} {}", path, err);
            }
        }
    }

    println!(
        "\ndone, total: {} group members, skipped: {}, total bytes: {}, took: {:?}\n",
        count.separate_with_commas(),
        skipped.separate_with_commas(),
        humanize_bytes_binary!(total_bytes),
        Instant::now().duration_since(start),
    );

    Ok(())
}
