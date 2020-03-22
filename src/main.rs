#![warn(rust_2018_idioms)]

mod errors;
mod explain;
mod graph;
mod pgpass;

use errors::*;
use explain::*;
use pgpass::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// this option executes explain analyse
    /// /!\ Be carful, that executes the query!
    #[structopt(long)]
    analyse: bool,
    /// Specifies the command to execute
    #[structopt(short, long)]
    command: Option<String>,
    /// Specifies the name of the database to connect to
    dbname: Option<String>,
    /// Don’t execute the query, the input is already an explain plan in JSON
    #[structopt(short = "n", long)]
    dry_run: bool,
    /// Read commands from the file, rather than standard input
    #[structopt(short, long)]
    file: Option<String>,
    /// Specifies the host name of the machine on which the server is running
    #[structopt(short, long)]
    host: Option<String>,
    /// Put output into file
    #[structopt(short, long)]
    output: Option<String>,
    /// Prompt for a password before connecting to a database
    #[structopt(short = "W", long)]
    password: bool,
    /// Specifies the TCP port on which the server is listening for connections
    #[structopt(short, long)]
    port: Option<String>,
    /// Connect to the database as the user
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

        let mut client = try_connect(&mut config(&opt)?, opt.password)?;

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

fn try_connect(config: &mut postgres::config::Config, retry: bool) -> Result<postgres::Client> {
    match connect(config, false) {
        Ok(client) => Ok(client),
        Err(err) => {
            if retry && &format!("{}", err) == "invalid configuration: password missing" {
                Ok(connect(config, true)?)
            } else {
                Err(err.into())
            }
        }
    }
}

fn connect(
    config: &mut postgres::config::Config,
    ask_password: bool,
) -> std::result::Result<postgres::Client, postgres::Error> {
    if ask_password {
        let password = rpassword::prompt_password_stdout("Password: ").unwrap();

        config.password(password.trim_matches('\n'));
    }

    let client = config.connect(postgres::NoTls)?;

    Ok(client)
}

fn config(opt: &Opt) -> Result<postgres::config::Config> {
    let mut config = postgres::config::Config::new();

    let host = opt
        .host
        .clone()
        .or_else(|| std::env::var("PGHOST").ok())
        .unwrap_or_else(|| "/run/postgresql".to_string());
    config.host(&host);

    let user = opt
        .user
        .clone()
        .or_else(|| std::env::var("PGUSER").ok())
        .unwrap_or_else(|| std::env::var("USER").unwrap());
    config.user(&user);

    let dbname = opt
        .dbname
        .clone()
        .or_else(|| std::env::var("PGDATABASE").ok())
        .unwrap_or_else(|| user.clone());
    config.dbname(&dbname);

    let port = opt
        .port
        .clone()
        .or_else(|| std::env::var("PGPORT").ok())
        .unwrap_or_else(|| "5432".to_string())
        .parse()?;
    config.port(port);

    let pgpass = PgPass::from_file();

    if let Some(password) = pgpass.find(&host, port, &dbname, &user) {
        config.password(&password);
    }

    Ok(config)
}
