use std::error::Error;
use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::net::Ipv4Addr;
use std::str::FromStr;

// Our configuration module
pub mod config;

// The hosts file of the platform
#[cfg(target_os = "macos")]
const HOSTS_PATH: &str = "/private/etc/hosts";

#[cfg(target_os = "linux")]
const HOSTS_PATH: &str = "/etc/hosts";

#[cfg(target_os = "windows")]
const HOSTS_PATH: &str = "C:\\Windows\\System32\\Drivers\\etc\\hosts";

pub fn run(config: &config::Config) -> Result<(), Box<dyn Error>> {
    // First read the hosts file
    let hosts_file = read_hosts_file()?;
    let mut website_collection = WebsiteCollection::from(hosts_file.as_str());

    if config.command == "list" {
        list_blocked(&website_collection);
        Ok(())
    } else if config.command == "block" {
        website_collection.add(Website {
            domain: config.domain.clone().unwrap().to_string(),
            redirect_ip: config.redirect_ip.unwrap(),
            is_blocked: true,
        })?;
        fs::write(HOSTS_PATH, website_collection.to_string())?;
        Ok(())
    } else if config.command == "unblock" {
        website_collection.remove(&config.domain.as_ref().unwrap())?;
        fs::write(HOSTS_PATH, website_collection.to_string())?;
        Ok(())
    } else {
        return Err("Command not found!".into());
    }

    // After finishing write the string value of website_collection
    // to the hosts file
}

fn list_blocked(website_collection: &WebsiteCollection) {
    if website_collection.collection.len() == 0 {
        println!("No Websites were found!");
    } else {
        website_collection.collection.iter().for_each(|el| {
            println!("{}", el);
        })
    }
}

// Convert hosts file into a vector list
fn read_hosts_file() -> Result<String, String> {
    match fs::read_to_string(HOSTS_PATH) {
        Ok(str) => Ok(str),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                return Err("Hosts file couldn't be found".to_string());
            }
            ErrorKind::PermissionDenied => {
                return Err("Please run the app with elevated privileges!".to_string());
            }
            _ => {
                return Err("Failed to read hosts file.".to_string());
            }
        },
    }
}
// A struct that manages the collection of website
// objects in a correct manner
#[derive(Debug, PartialEq)]
struct WebsiteCollection {
    metadata: String,
    collection: Vec<Website>,
}

impl WebsiteCollection {
    // pub fn new(collection: Vec<Website>) -> WebsiteCollection {
    //     WebsiteCollection {
    //         metadata: String::new(),
    //         collection,
    //     }
    // }

    pub fn add(&mut self, website: Website) -> Result<(), String> {
        // First check if there is no site that exists
        // containing this domain
        if self.collection.iter().any(|el| el.domain == website.domain) {
            return Err(format!(
                "The website with the domain {} already exists.",
                website.domain
            ));
        }
        self.collection.push(website);
        Ok(())
    }

    pub fn remove(&mut self, domain_name: &str) -> Result<(), String> {
        // Check if the domain_name of the website exists
        if !self.collection.iter().any(|el| el.domain == domain_name) {
            return Err(format!(
                "There is no website with a domain name '{}' that is currently blocked.",
                domain_name
            ));
        }
        let index_of_unwanted_website = self
            .collection
            .iter()
            .position(|el| el.domain == domain_name)
            .unwrap();
        self.collection.swap_remove(index_of_unwanted_website);
        Ok(())
    }
}

impl fmt::Display for WebsiteCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We are going to simply return the metadata and
        // the collection turned into a String combined.
        let mut result = self.metadata.clone();
        self.collection.iter().for_each(|el| {
            if el.is_blocked {
                result += "#!wb\n";
                result += &(el.to_string() + "\n");
            }
        });
        write!(f, "{}", result)
    }
}

impl From<&str> for WebsiteCollection {
    fn from(hosts_doc: &str) -> Self {
        let mut valid_website_line_indices = Vec::new();

        let parsed_vec = hosts_doc
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                if index == 0 {
                    None
                } else if hosts_doc.lines().nth(index - 1).unwrap().trim() == "#!wb" {
                    match Website::from_string(line.trim()) {
                        Ok(website) => {
                            valid_website_line_indices.push(index - 1);
                            valid_website_line_indices.push(index);
                            Some(website)
                        }
                        Err(_) => None,
                    }
                } else {
                    None
                }
            })
            .collect();

        let metadata: String = hosts_doc
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                if valid_website_line_indices.contains(&index) {
                    None
                } else {
                    Some(line.to_string() + "\n")
                }
            })
            .collect();

        WebsiteCollection {
            metadata,
            collection: parsed_vec,
        }
    }
}

// This struct represents a website.
// mapped to a site that the user wants
// to redirect to.
#[derive(Debug, PartialEq)]
struct Website {
    domain: String,
    redirect_ip: Ipv4Addr,
    is_blocked: bool,
}

impl Website {
    // fn new(domain: String, redirect_ip: Ipv4Addr) -> Website {
    //     // Create a new instance of the Website struct from
    //     // function parameters
    //     Website {
    //         domain,
    //         redirect_ip,
    //         is_blocked: false,
    //     }
    // }

    fn from_string(value: &str) -> Result<Website, String> {
        // Generate a website object using the string value
        // provided. Validate the ip address also
        let mut string_parts = value.split_whitespace().into_iter();

        let redirect_ip = match string_parts.next() {
            Some(ip_addr) => match Ipv4Addr::from_str(ip_addr) {
                Ok(ip_addr) => ip_addr,
                Err(_) => return Err("Invalid IP address!".to_string()),
            },
            None => {
                return Err("No redirect IP provided!".to_string());
            }
        };

        let domain_name = match string_parts.next() {
            Some(dom_name) => dom_name,
            None => {
                return Err("No domain name provided!".to_string());
            }
        };

        Ok(Website {
            domain: domain_name.to_string(),
            redirect_ip,
            is_blocked: true,
        })
    }

    // fn block(&mut self) {
    //     self.is_blocked = true;
    // }

    // fn unblock(&mut self) {
    //     self.is_blocked = false;
    // }
}

impl fmt::Display for Website {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.redirect_ip, self.domain)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_from_string_on_website_collection() {
        let hosts_file_contents = "\
# This website shouldn't be considered as a website
# Since there is no `#!wb` before it.
0.0.0.0 google.com

# The next will be
#!wb
127.0.0.1 localhost

# And this won't since there's an invalid
# IP address.
#!wb
8.8.8.8.9 dns
";
        let the_collection = WebsiteCollection {
            collection: vec![Website {
                domain: String::from("localhost"),
                redirect_ip: Ipv4Addr::new(127, 0, 0, 1),
                is_blocked: true,
            }],
            metadata: "\
# This website shouldn't be considered as a website
# Since there is no `#!wb` before it.
0.0.0.0 google.com

# The next will be

# And this won't since there's an invalid
# IP address.
#!wb
8.8.8.8.9 dns
"
            .to_string(),
        };

        assert_eq!(the_collection, WebsiteCollection::from(hosts_file_contents))
    }

    #[test]
    fn test_to_string_on_website_collection() {
        let the_collection = WebsiteCollection {
            collection: vec![Website {
                domain: String::from("localhost"),
                redirect_ip: Ipv4Addr::new(127, 0, 0, 1),
                is_blocked: true,
            }],
            metadata: "\
# This website shouldn't be considered as a website
# Since there is no `#!wb` before it.
0.0.0.0 google.com

# The next will be

# And this won't since there's an invalid
# IP address.
#!wb
8.8.8.8.9 dns\n"
                .to_string(),
        };

        assert_eq!(
            "\
# This website shouldn't be considered as a website
# Since there is no `#!wb` before it.
0.0.0.0 google.com

# The next will be
#!wb
127.0.0.1 localhost

# And this won't since there's an invalid
# IP address.
#!wb
8.8.8.8.9 dns
",
            the_collection.to_string()
        );
    }
}
