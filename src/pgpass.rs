struct Auth {
    hostname: String,
    port: String,
    database: String,
    username: String,
    password: String,
}

impl Auth {
    fn split<'a>(line: &'a str) -> Vec<&'a str> {
        let mut tokens = Vec::new();
        let mut slash = false;
        let mut last_pos = 0;

        for (pos, c) in line.char_indices() {
            if c == ':' && !slash {
                tokens.push(line.get(last_pos..pos).unwrap());

                last_pos = pos + 1;
            }

            slash = c == '\\' && !slash;
        }

        tokens.push(line.get(last_pos..).unwrap());

        tokens
    }

    fn unescape(s: &str) -> String {
        s.replace("\\\\", "\\").replace("\\:", ":")
    }
}

impl std::convert::TryFrom<&str> for Auth {
    type Error = ();

    fn try_from(line: &str) -> std::result::Result<Self, Self::Error> {
        let tokens = Self::split(line);

        if tokens.len() != 5 {
            return Err(());
        }

        let auth = Auth {
            hostname: Self::unescape(tokens[0]),
            port: Self::unescape(tokens[1]),
            database: Self::unescape(tokens[2]),
            username: Self::unescape(tokens[3]),
            password: Self::unescape(tokens[4]),
        };

        Ok(auth)
    }
}

pub(crate) struct PgPass {
    entries: Vec<Auth>,
}

impl PgPass {
    fn from_str(pgpass: &str) -> Self {
        use std::convert::TryInto;

        let entries = pgpass
            .split('\n')
            .filter(|e| !e.trim_start().starts_with('#'))
            .filter_map(|e| e.try_into().ok())
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

    #[test]
    fn parse_escape() {
        let pgpass = PgPass::from_str("local\\:host:*:post\\\\gres:*:1234");

        assert_eq!(
            Some("1234".to_string()),
            pgpass.find("local:host", 5432, "post\\gres", "postgres")
        );
    }
}
