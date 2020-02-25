use csv::Writer;
use postgres::{Client, NoTls, SimpleQueryMessage};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub fn export(db: &str, file_name: PathBuf, sql: &str) -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect(db, NoTls)?;

    let rows = client.simple_query(sql).map_err(|err| Box::new(err))?;

    let mut writer = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_path(&file_name)
        .map_err(|err| Box::new(err))?;

    save(&rows, &mut writer)?;

    writer.flush().map_err(|err| Box::new(err))?;

    println!("Saved file to: {}", file_name.display());

    Ok(())
}

fn save(rows: &[SimpleQueryMessage], writer: &mut Writer<File>) -> Result<(), Box<dyn Error>> {
    for row in rows {
        if let SimpleQueryMessage::Row(simple_row) = row {
            writer
                .write_record((0..simple_row.len()).into_iter().map(|column_index| {
                    simple_row
                        .try_get(column_index)
                        .unwrap_or_default()
                        .unwrap_or_default()
                }))
                .map_err(Box::new)?;
        }
    }
    Ok(())
}
