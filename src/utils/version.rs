pub struct SystemInfo {
    pub(crate) name: String,
    pub(crate) codename: String,
    pub(crate) id: String,
    pub(crate) pretty_name: String,
    pub(crate) ansi_color: String,
    pub(crate) home_url: String,
    pub(crate) support_url: String,
    pub(crate) bug_report_url: String,
    pub(crate) version_id: String,
    pub(crate) license: String
}

impl SystemInfo {
    pub(crate) fn from_string(input: &str) -> Self {
        let mut system_info = SystemInfo {
            name: String::new(),
            codename: String::new(),
            id: String::new(),
            pretty_name: String::new(),
            ansi_color: String::new(),
            home_url: String::new(),
            support_url: String::new(),
            bug_report_url: String::new(),
            version_id: String::new(),
            license: String::new()
        };

        for line in input.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                continue;
            }
            match parts[0] {
                "NAME" => system_info.name = parts[1].trim().to_string(),
                "CODENAME" => system_info.codename = parts[1].trim().to_string(),
                "ID" => system_info.id = parts[1].trim().to_string(),
                "PRETTY_NAME" => system_info.pretty_name = parts[1].trim().to_string(),
                "ANSI_COLOR" => system_info.ansi_color = parts[1].trim().to_string(),
                "HOME_URL" => system_info.home_url = parts[1].trim().to_string(),
                "SUPPORT_URL" => system_info.support_url = parts[1].trim().to_string(),
                "BUG_REPORT_URL" => system_info.bug_report_url = parts[1].trim().to_string(),
                "VERSION_ID" => system_info.version_id = parts[1].trim().to_string(),
                "LICENSE" => system_info.license = parts[1].trim().to_string(),
                _ => (),
            }
        }

        system_info
    }
}