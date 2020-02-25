use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    println!("{:#?}", &opt);

    if let Err(err) = sql2csv::export(&opt.db, opt.out, &opt.sql) {
        eprintln!("Export Error: {}", err);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sql2csv")]
struct Opt {
    #[structopt(short, long)]
    db: String,

    #[structopt(short, long, parse(from_os_str))]
    out: PathBuf,

    #[structopt()]
    sql: String,
}
