#![warn(rust_2018_idioms)]

mod errors;
mod explain;
mod graph;

use errors::*;
use explain::*;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
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

impl From<Opt> for elephantry::Config {
    fn from(opt: Opt) -> Self {
        Self {
            host: opt.host,
            user: opt.user,
            dbname: opt.dbname,
            port: opt.port,

            ..Default::default()
        }
    }
}

fn main() -> Result {
    human_panic::setup_panic!();

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

        let client = try_connect(&opt)?;

        let results = client.execute(explain_query.as_str())?;
        results.get(0).get("\"QUERY PLAN\"")
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

fn try_connect(opt: &Opt) -> elephantry::Result<elephantry::Pool> {
    if let Ok(client) = elephantry::Pool::new(&opt.dbname.clone().expect("No database name given"))
    {
        Ok(client)
    } else {
        {
            let mut config = opt.clone().into();

            match elephantry::Pool::from_config(&config) {
                Ok(client) => Ok(client),
                Err(err) => {
                    if opt.password
                        && &format!("{}", err) == "invalid configuration: password missing"
                    {
                        let password = rpassword::prompt_password_stdout("Password: ").unwrap();
                        config.password = Some(password.trim_matches('\n').to_string());

                        elephantry::Pool::from_config(&config)
                    } else {
                        Err(err)
                    }
                }
            }
        }
    }
}
