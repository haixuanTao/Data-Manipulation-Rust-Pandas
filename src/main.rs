use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::result::Result;
mod lib;
use itertools::Itertools;

fn inspect(path: &str) {
    let mut record: lib::Record = HashMap::new();

    let mut rdr = csv::Reader::from_path(path).unwrap();

    for result in rdr.deserialize() {
        match result {
            Ok(rec) => {
                record = rec;
                break;
            }
            Err(_e) => (),
        };
    }
    // Print Struct
    println!("#[skip_serializing_none]");
    println!("#[derive(Debug, Deserialize, Serialize)]");
    println!("struct lib::DataFrame {{");
    for (key, value) in &record {
        println!("    #[serialize_always]");

        match value.parse::<i64>() {
            Ok(_n) => {
                // println!("    #[serde(deserialize_with = \"empty_string_as_none\")]");
                println!("    {}: Option<i64>,", key);
                continue;
            }
            Err(_e) => (),
        }
        match value.parse::<f64>() {
            Ok(_n) => {
                // println!("    #[serde(deserialize_with = \"empty_string_as_none\")]");
                println!("    {}: Option<f64>,", key);
                continue;
            }
            Err(_e) => (),
        }
        println!("    {}: Option<String>,", key);
    }
    println!("}}");
}

fn struct_oriented_result(
    path: &str,
    path_country: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(transcoded); //https://stackoverflow.com/questions/53826986/how-to-read-a-non-utf8-encoded-csv-file

    // let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut records: Vec<lib::DataFrame> = Vec::new();

    for result in rdr.deserialize() {
        match result {
            Ok(rec) => {
                records.push(rec);
            }
            Err(e) => println!("{}", e),
        };
    }

    // 1. Filtering
    // records.retain(|x| x.country_txt.as_ref().unwrap() == "United States");

    // let mut wtr =
    //     csv::Writer::from_path("/home/peter/Documents/TEST/RUST/terrorism/output_rust_filter.csv")?;

    // for record in &records {
    //     wtr.serialize(record)?;
    // }

    // 2. Group By
    let groups = records
        .into_iter()
        // .sorted_unstable_by(|a, b| Ord::cmp(&a.country_txt, &b.country_txt))
        .group_by(|record| record.country_txt.clone())
        .into_iter()
        .map(|(country, group)| {
            let (total_nkill, count, average_individual) = group.into_iter().fold(
                (0., 0., 0.),
                |(total_nkill, count, average_individual), record| {
                    (
                        total_nkill + record.nkill.unwrap_or(0.),
                        count + 1.,
                        average_individual + record.individual.unwrap_or(0.),
                    )
                },
            );
            lib::GroupBy {
                country: country.unwrap(),
                total_nkill,
                average_individual: average_individual / count,
                count,
            }
        })
        .collect::<Vec<_>>();
    let mut wtr =
        csv::Writer::from_path("/home/peter/Documents/TEST/RUST/terrorism/output_rust_groupby.csv")
            .unwrap();

    for group in &groups {
        wtr.serialize(group)?;
    }
    // 3. Mutation

    // records.iter_mut().for_each(|x: &mut lib::DataFrame| {
    //     let nkill = match x.nkill.as_ref() {
    //         Some(nkill) => nkill,
    //         None => &0.,
    //     };

    //     x.computed = Some((nkill - 10.) / 2. + nkill * nkill / 3.);
    // });

    // let mut wtr = csv::Writer::from_path(
    //     "/home/peter/Documents/TEST/RUST/terrorism/output_rust_mutation.csv",
    // )?;
    // for record in &records {
    //     wtr.serialize(record)?;
    // }

    // 4. Join
    // let mut records_country: Vec<lib::DataFrameCountry> = Vec::new();
    // let file = File::open(path_country)?;
    // let transcoded = DecodeReaderBytesBuilder::new()
    //     .encoding(Some(WINDOWS_1252))
    //     .build(file);
    // let mut rdr = csv::ReaderBuilder::new()
    //     .delimiter(b',')
    //     .from_reader(transcoded); //https://stackoverflow.com/questions/53826986/how-to-read-a-non-utf8-encoded-csv-file

    // for result in rdr.deserialize() {
    //     match result {
    //         Ok(rec) => {
    //             records_country.push(rec);
    //         }
    //         Err(e) => println!("{}", e),
    //     };
    // }

    // for country in records_country {
    //     records
    //         .iter_mut()
    //         .filter(|record| record.country_txt == country.Short_Name)
    //         .for_each(|x| {
    //             x.add_country_ext(Some(country.clone()));
    //         });
    // }
    // let mut wtr =
    //     csv::Writer::from_path("/home/peter/Documents/TEST/RUST/terrorism/output_rust_join.csv")
    //         .unwrap();
    // for record in &records {
    //     wtr.serialize(record)?;
    // }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/home/peter/Documents/TEST/RUST/terrorism/src/globalterrorismdb_0718dist.csv";
    let path_country = "/home/peter/Documents/TEST/RUST/terrorism/src/WDICountry.csv";
    // inspect_schema(path);
    struct_oriented_result(path, path_country)
}
