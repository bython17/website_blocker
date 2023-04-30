use std::net::Ipv4Addr;
use std::str::FromStr;

// Parse user arguments.
pub struct Config {
    pub command: String,
    pub redirect_ip: Option<Ipv4Addr>,
    pub domain: Option<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // For the program name
        args.next();

        // Collect our arguments
        let command = match args.next() {
            Some(cmd) => cmd,
            None => return Err("No command was supplied"),
        };

        let domain_name = match args.next() {
            Some(val) => Some(val),
            None => {
                if command != "list" {
                    return Err("No Redirect IP is provided!");
                }
                None
            }
        };

        let redirect_ip = match args.next() {
            Some(val) => {
                // try ot change the val to an IP address
                match Ipv4Addr::from_str(&val) {
                    Ok(ip) => Some(ip),
                    Err(_) => return Err("Invalid IP address"),
                }
            }
            None => {
                if command == "block" {
                    return Err("No Redirect IP is provided!");
                }
                None
            }
        };

        Ok(Config {
            command,
            redirect_ip,
            domain: domain_name,
        })
    }
}
