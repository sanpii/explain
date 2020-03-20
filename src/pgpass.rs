struct Auth {
    hostname: String,
    port: String,
    database: String,
    username: String,
    password: String,
}

pub(crate) struct PgPass {
    entries: Vec<Auth>,
}

impl PgPass {
    fn from_str(pgpass: &str) -> Self {
        let entries = pgpass
            .split('\n')
            .filter(|e| !e.trim_start().starts_with('#'))
            .filter_map(|e| {
                let tokens = e.split(':').collect::<Vec<_>>();

                if tokens.len() != 5 {
                    return None;
                }

                let auth = Auth {
                    hostname: tokens[0].to_string(),
                    port: tokens[1].to_string(),
                    database: tokens[2].to_string(),
                    username: tokens[3].to_string(),
                    password: tokens[4].to_string(),
                };

                Some(auth)
            })
            .collect();

        Self { entries }
    }

    pub fn from_file() -> Self {
        let filename = std::env::var("PGPASSFILE").unwrap_or_else(|_| "~/.pgpass".to_string());

        let contents = std::fs::read_to_string(filename).unwrap_or_else(|_| String::new());

        Self::from_str(&contents)
    }

    pub fn find(
        &self,
        hostname: &str,
        port: u16,
        database: &str,
        username: &str,
    ) -> Option<String> {
        for entry in &self.entries {
            if &entry.hostname != "*" && entry.hostname != hostname {
                continue;
            }

            if &entry.port != "*" && entry.port != port.to_string() {
                continue;
            }

            if &entry.database != "*" && entry.database != database {
                continue;
            }

            if &entry.username != "*" && entry.database != username {
                continue;
            }

            return Some(entry.password.clone());
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::pgpass::PgPass;

    #[test]
    fn parse() {
        PgPass::from_str("*:*:*:*:1234");
    }

    #[test]
    fn parse_invalid() {
        PgPass::from_str("invalid");
    }

    #[test]
    fn parse_comment() {
        PgPass::from_str("# comment");
    }

    #[test]
    fn find() {
        let pgpass = PgPass::from_str("localhost:5432:postgres:postgres:1234");

        assert_eq!(
            Some("1234".to_string()),
            pgpass.find("localhost", 5432, "postgres", "postgres")
        );
    }

    #[test]
    fn not_find() {
        let pgpass = PgPass::from_str("localhost:5431:postgres:postgres:1234");

        assert_eq!(None, pgpass.find("localhost", 5432, "postgres", "postgres"));
    }

    #[test]
    fn not_wilcard() {
        let pgpass = PgPass::from_str("*:*:*:*:1234");

        assert_eq!(
            Some("1234".to_string()),
            pgpass.find("localhost", 5432, "postgres", "postgres")
        );
    }
}
