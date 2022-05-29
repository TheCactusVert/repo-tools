#[derive(Default)]
pub struct Package {
    pub filename: String,
}

impl Package {
    pub fn from_str<'a>(str: &'a str) -> Self {
        let mut pkg: Self = Self::default();
        let mut section: Option<&'a str> = None;

        for line in str.lines() {
            if line.is_empty() {
                continue;
            }

            if line.starts_with('%') && line.ends_with('%') {
                section = Some(line);
            } else {
                match section {
                    Some("%FILENAME%") => {
                        pkg.filename = line.to_string();
                    }
                    _ => {}
                }
            }
        }

        pkg
    }
}
