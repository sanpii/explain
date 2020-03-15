#![warn(rust_2018_idioms)]

mod errors;
mod explain;
mod graph;

use errors::*;
use explain::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// this option execute explain analyse
    /// /!\ Be carful, that execute the query!
    #[structopt(long)]
    analyse: bool,
    #[structopt(short, long)]
    command: Option<String>,
    dbname: Option<String>,
    /// Don’t execute the query, the input is already an explain plan in JSON
    #[structopt(short = "n", long)]
    dry_run: bool,
    #[structopt(short, long)]
    file: Option<String>,
    #[structopt(short, long)]
    host: Option<String>,
    #[structopt(short, long)]
    output: Option<String>,
    #[structopt(short = "W", long)]
    password: bool,
    #[structopt(short, long)]
    port: Option<String>,
    #[structopt(short = "U", long)]
    user: Option<String>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let query = match (&opt.command, &opt.file) {
        (Some(query), None) => query.clone(),
        (None, Some(file)) => std::fs::read_to_string(file)?,
        (None, None) => {
            let mut buffer = String::new();

            std::io::stdin().read_line(&mut buffer)?;

            buffer
        }
        (Some(_), Some(_)) => panic!("Require command or file, not both"),
    };

    let json = if opt.dry_run {
        serde_json::from_str(&query)?
    } else {
        let analyse = if opt.analyse { ", analyse" } else { "" };
        let explain_query = format!(
            "explain (format json, costs, verbose, summary{}) {}",
            analyse, query
        );

        let mut client = try_connect(&dsn(&opt), opt.password)?;

        let results = client.query_one(explain_query.as_str(), &[])?;
        results.get("QUERY PLAN")
    };

    let explains: Vec<Explain> = serde_json::from_value(json)?;
    let graph = graph::dot(&explains[0]);

    if let Some(output) = opt.output {
        use std::io::Write;

        let mut output = std::fs::File::create(output)?;
        output.write_all(graph.as_bytes())?;
    } else {
        print!("{}", graph);
    }

    Ok(())
}

fn try_connect(dsn: &str, retry: bool) -> Result<postgres::Client> {
    match connect(&dsn, false) {
        Ok(client) => Ok(client),
        Err(err) => {
            if retry && &format!("{}", err) == "invalid configuration: password missing" {
                Ok(connect(dsn, true)?)
            } else {
                Err(err.into())
            }
        }
    }
}

fn connect(
    dsn: &str,
    ask_password: bool,
) -> std::result::Result<postgres::Client, postgres::Error> {
    let dsn = if ask_password {
        let password = rpassword::prompt_password_stdout("Password: ").unwrap();

        format!("{} password={}", dsn, password.trim_matches('\n'))
    } else {
        dsn.to_string()
    };

    let client = postgres::Client::connect(&dsn, postgres::NoTls)?;

    Ok(client)
}

fn dsn(opt: &Opt) -> String {
    let host = opt
        .host
        .clone()
        .or_else(|| std::env::var("PGHOST").ok())
        .unwrap_or_else(|| "/run/postgresql".to_string());
    let user = opt
        .user
        .clone()
        .or_else(|| std::env::var("PGUSER").ok())
        .unwrap_or_else(|| std::env::var("USER").unwrap());
    let dbname = opt
        .dbname
        .clone()
        .or_else(|| std::env::var("PGDATABASE").ok())
        .unwrap_or_else(|| user.clone());

    let mut dsn = format!("host={} user={} dbname={}", host, user, dbname);

    let port = opt
        .port
        .clone()
        .or_else(|| std::env::var("PGPORT").ok());

    if let Some(port) = port {
        dsn.push_str(&format!(" port={}", port));
    }

    dsn
}
