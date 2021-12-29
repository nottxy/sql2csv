use dotenv::dotenv;
use serde::Deserialize;
use sql2csv::Query;
use std::{env, fs, path::PathBuf};
use structopt::StructOpt;

fn main() {
    dotenv().ok();

    let opt = Opt::from_args();

    let query = match opt.to_query() {
        Ok(query) => query,
        Err(err) => {
            eprintln!("EXPORT ERROR: {}", err);
            return;
        }
    };

    if let Err(err) = query.export() {
        eprintln!("EXPORT ERROR: {}", err);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sql2csv")]
enum Opt {
    File(OptFile),
    Inline(OptSql),
}

#[derive(StructOpt, Debug)]
struct OptFile {
    #[structopt(long)]
    file: String,
    #[structopt(long)]
    db: Option<String>,
}

#[derive(StructOpt, Debug, Deserialize)]
struct OptSql {
    #[structopt(long)]
    header: String,
    #[structopt(long)]
    sql: String,
    #[structopt(long, parse(from_os_str))]
    out: PathBuf,
    #[structopt(long)]
    db: Option<String>,
}

impl Opt {
    fn to_query(self) -> Result<Query, String> {
        let db_url = env::var("DATABASE_URL").ok();

        match self {
            Opt::File(opt_file) => opt_file.to_query(db_url),
            Opt::Inline(opt_sql) => opt_sql.to_query(db_url),
        }
    }
}

impl OptFile {
    fn to_query(self, db_url: Option<String>) -> Result<Query, String> {
        let file_content = match fs::read_to_string(&self.file) {
            Ok(file_content) => file_content,
            Err(err) => {
                let err_msg = format!("Read file err: {:?}", err);
                return Err(err_msg);
            }
        };

        let opt_sql: OptSql = match toml::from_str(&file_content) {
            Ok(opt_sql) => opt_sql,
            Err(err) => {
                let err_msg = format!("Deserialize file({}) err: {:?}", &self.file, err);
                return Err(err_msg);
            }
        };

        let db = self.db.or_else(|| db_url);

        opt_sql.to_query(db)
    }
}

impl OptSql {
    fn to_query(self, db_url: Option<String>) -> Result<Query, String> {
        let db = match self.db.or_else(|| db_url) {
            Some(db) => db,
            None => {
                return Err("The db should be set".to_string());
            }
        };

        Ok(Query::new(db, self.sql, self.header, self.out))
    }
}
