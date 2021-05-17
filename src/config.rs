use clap::{Arg, App};
use std::env;
use std::ffi::OsString;

#[derive(Debug)]
pub struct AppConfig {
    hostname: String,
    dns_server: Vec<String>,
}

pub fn parse_resolv_conf(resolv_conf_path: String) -> Vec<String> {
    let contents = std::fs::read_to_string(resolv_conf_path);
    let mut nameservers = vec![];
    if let Err(_) = contents {
        return nameservers;
    }
    let lines = contents.unwrap();

    for line in lines.split('\n') {
        if line.starts_with("nameserver ") {
            let nameserver_line = line.strip_prefix("nameserver ").unwrap();
            nameservers.push(nameserver_line.to_string());
        }
    }

    nameservers
}

impl AppConfig {
    pub fn from<I, T>(args: I) -> Self
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone
    {
        let app = App::new("dig-rs")
            .version("0.1")
            .author("Peter Malmgren <ptmalmgren@gmail.com>")
            .about("Rust version of dig")
            .arg(
                Arg::with_name("hostname")
                    .required(true)
                    .index(1)
            )
            .arg(
                Arg::with_name("global-server")
                    .required(false)
                    .takes_value(true)
                    .multiple(false)
                    .long("global-server")
            );

        let matches = app.get_matches_from(args);
        let resolv_conf_path = env::var_os("DNS_FILE")
            .map(|v| v.to_str().unwrap().to_string())
            .unwrap_or("/etc/resolv.conf".to_string());
        let hostname: String = matches.value_of("hostname").unwrap().to_string();
        let dns_server = matches
            .value_of("global-server")
            .map(|r: &str| Vec::from([r.to_string()]))
            .unwrap_or_else(|| parse_resolv_conf(resolv_conf_path));
        AppConfig {
            hostname,
            dns_server,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_parses_matches() {
        let app_config = AppConfig::from(vec!["dig-rs", "--global-server", "8.8.8.8", "google.com"].iter());
        assert_eq!(app_config.hostname, "google.com".to_string());
        assert_eq!(app_config.dns_server, vec!["8.8.8.8".to_string()]);
    }

    #[test]
    fn test_it_parses_resolv_conf() {
        std::env::set_var("DNS_FILE", "test/resolv.conf");
        let app_config = AppConfig::from(vec!["dig-rs", "google.com"].iter());
        assert_eq!(app_config.hostname, "google.com".to_string());
        assert_eq!(app_config.dns_server, vec!["1.1.1.1".to_string()]);
    }
}
