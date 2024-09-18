// use croaring::Bitmap;
use humanize_bytes::humanize_bytes_binary;
use roaring::bitmap::RoaringBitmap;
use rustc_hash::FxHashMap;
use std::error::Error;
use std::io::Write;
use std::time::Instant;
use thousands::Separable;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let mut bitmaps = FxHashMap::<u32, Vec<u32>>::default();

    let mut count: u32 = 0;
    let mut skipped: u32 = 0;

    let mut reader = csv::ReaderBuilder::new().from_path("../group_members.csv")?;
    println!("input file opened");
    let mut record = csv::StringRecord::new();
    while reader.read_record(&mut record)? {
        if record.len() != 2 || record[0].is_empty() || record[1].is_empty() {
            skipped += 1;
            continue;
        }

        let group_id = record[0].parse::<u32>().unwrap();
        let member_id = record[1].parse::<u32>().unwrap();

        count += 1;

        let bitmap = bitmaps.entry(group_id).or_default();
        bitmap.push(member_id);
    }

    println!("bitmaps: {}", bitmaps.len());
    let mut total_bytes = 0;

    for (group_id, data) in bitmaps.iter_mut() {
        data.sort_unstable();
        // bitmap construction is the bottleneck, sorting and optimize the build is worth it for this dataset
        let bitmap = RoaringBitmap::from_sorted_iter(data.iter().copied()).unwrap();
        let mut bytes = vec![];
        // let bytes = bitmap.serialize_into_vec::<croaring::Native>(&mut bytes);
        bitmap.serialize_into(&mut bytes)?;

        total_bytes += bytes.len();

        println!(
            "group_id: {}, len: {:?}, size: {}",
            group_id,
            data.len(),
            // bitmap.cardinality(),
            humanize_bytes_binary!(bytes.len())
        );

        let path = format!("group_{}.roaring", group_id);

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path);

        match f {
            Ok(mut file) => file.write_all(&bytes)?,
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
