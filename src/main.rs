use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    if let Err(err) = sql2csv::export(&opt.db, opt.out, &opt.sql, &opt.header) {
        eprintln!("Export Error: {}", err);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sql2csv")]
struct Opt {
    #[structopt(long)]
    db: String,

    #[structopt(long, parse(from_os_str))]
    out: PathBuf,

    #[structopt(long)]
    header: String,

    #[structopt(long)]
    sql: String,
}
