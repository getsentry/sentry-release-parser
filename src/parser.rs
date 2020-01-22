use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref RELEASE_REGEX: Regex = Regex::new(r#"^(@?[^@]+)@(.*?)$"#).unwrap();
    static ref SEMVER_REGEX: Regex = Regex::new(r#"(?x)
        ^
            (?P<major>0|[1-9][0-9]*)\.
            (?P<minor>0|[1-9][0-9]*)\.
            (?P<patch>0|[1-9][0-9]*)
            (?:-(?P<prerelease>(?:0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*)
            (?:\.(?:0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*))*))?
            (?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?
        $
    "#).unwrap();
}

pub struct Release<'a> {
    raw: &'a str,
    package: &'a str,
    version_raw: &'a str,
}

impl<'a> Release<'a> {
    pub fn new(release: &'a str) -> Release<'a> {
        if let Some(caps) = RELEASE_REGEX.captures(release) {
            Release {
                raw: release,
                package: caps.get(1).unwrap().as_str(),
                version_raw: caps.get(2).unwrap().as_str(),
            }
        } else {
            Release {
                raw: release,
                package: "",
                version_raw: release,
            }
        }
    }

    pub fn raw(&self) -> &str {
        self.raw
    }

    pub fn package(&self) -> &str {
        self.package
    }

    pub fn version_raw(&self) -> &str {
        self.version_raw
    }
}