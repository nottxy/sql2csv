use serde::Deserialize;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    dbg!(&opt);

    let sql = match read_header_sql(&opt.input) {
        Some(sql) => sql,
        None => return,
    };

    if let Err(err) = sql2csv::export(&opt.db, opt.out, &sql.sql, &sql.header) {
        eprintln!("Export Error: {}", err);
    }
}

fn read_header_sql(input: &Input) -> Option<Sql> {
    match input {
        Input::File { file } => {
            let file_content = fs::read_to_string(file).ok()?;
            toml::from_str(&file_content).ok()
        }
        Input::Inline(sql) => Some(sql.clone()),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sql2csv")]
struct Opt {
    #[structopt(long)]
    db: String,
    #[structopt(long, parse(from_os_str))]
    out: PathBuf,
    #[structopt(subcommand)]
    input: Input,
}

#[derive(StructOpt, Debug)]
enum Input {
    File {
        #[structopt(long)]
        file: String,
    },
    Inline(Sql),
}

#[derive(StructOpt, Debug, Deserialize, Clone)]
struct Sql {
    #[structopt(long)]
    header: String,
    #[structopt(long)]
    sql: String,
}
