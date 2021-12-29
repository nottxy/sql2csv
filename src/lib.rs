use csv::Writer;
use postgres::{Client, NoTls, SimpleQueryMessage};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub struct Query {
    db: String,
    sql: String,
    header: String,
    file_name: PathBuf,
}

impl Query {
    pub fn new(db: String, sql: String, header: String, file_name: PathBuf) -> Query {
        Query {
            db,
            sql,
            header,
            file_name,
        }
    }

    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        let mut client = Client::connect(&self.db, NoTls)?;

        let rows = client.simple_query(&self.sql)?;

        let mut writer = csv::WriterBuilder::new()
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_path(&self.file_name)?;

        write_header(&mut writer, &self.header)?;

        write_body(&mut writer, &rows)?;

        writer.flush()?;

        println!("Saved file to: {}", self.file_name.display());

        Ok(())
    }
}

fn write_header(writer: &mut Writer<File>, header: &str) -> Result<(), Box<dyn Error>> {
    writer.write_record(header.split(',').map(|cell| cell.trim()))?;
    Ok(())
}

fn write_body(
    writer: &mut Writer<File>,
    rows: &[SimpleQueryMessage],
) -> Result<(), Box<dyn Error>> {
    for row in rows {
        if let SimpleQueryMessage::Row(simple_row) = row {
            writer.write_record((0..simple_row.len()).map(|column_index| {
                simple_row
                    .try_get(column_index)
                    .unwrap_or_default()
                    .unwrap_or_default()
            }))?;
        }
    }
    Ok(())
}
