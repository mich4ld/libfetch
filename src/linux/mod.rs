use std::{fs, collections::HashMap,};

pub struct Linux {}

impl Linux {
    pub fn name(&self) -> Option<String> {
        let os_name = parse_release()?;
        let os_name = os_name.replace('\"', "");

        Some(os_name)
    }
}

fn parse_release() -> Option<String> {
    let mut parsed_map = HashMap::new();
    let release_file = fs::read_to_string("/etc/os-release").ok()?;
            
    for line in release_file.lines() {
        if line.is_empty() {
            continue;
        }

        let key_value = line.split("=")
            .map(|item| { item.trim() })
            .collect::<Vec<&str>>();

            // Expecting exacly 2 items: key and value
        if key_value.len() != 2 {
            continue;
        }

        parsed_map.insert(
            key_value[0],
            key_value[1].to_string(),
        );
    }

    let os_name = parsed_map.get("PRETTY_NAME");
    match os_name {
        Some(os) => {
           let result = os.clone();
                Some(result)
            }
        None => {
            let os = parsed_map.get("NAME")?;
            let result = os.clone();
            Some(result)
        }
    }

}
