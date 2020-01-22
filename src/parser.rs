use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RELEASE_REGEX: Regex = Regex::new(r#"^(@?[^@]+)@(.*?)$"#).unwrap();
    static ref VERSION_REGEX: Regex = Regex::new(
        r#"(?x)
        ^
            (?P<major>0|[1-9][0-9]*)
            (?:\.(?P<minor>0|[1-9][0-9]*))?
            (?:\.(?P<patch>0|[1-9][0-9]*))?
            (?:-?
                (?P<prerelease>(?:0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*)
                (?:\.(?:0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*))*))?
            (?:\+(?P<build_code>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?
        $
    "#
    )
    .unwrap();
}

pub struct Version<'a> {
    major: u64,
    minor: u64,
    patch: u64,
    pre: &'a str,
    build_code: &'a str,
}

impl<'a> Version<'a> {
    pub fn new(version: &'a str) -> Option<Version<'a>> {
        let caps = if let Some(caps) = VERSION_REGEX.captures(version) {
            caps
        } else {
            return None;
        };
        Some(Version {
            major: caps[1].parse().unwrap(),
            minor: caps
                .get(2)
                .and_then(|x| x.as_str().parse().ok())
                .unwrap_or(0),
            patch: caps
                .get(3)
                .and_then(|x| x.as_str().parse().ok())
                .unwrap_or(0),
            pre: caps.get(4).map(|x| x.as_str()).unwrap_or(""),
            build_code: caps.get(5).map(|x| x.as_str()).unwrap_or(""),
        })
    }

    pub fn major(&self) -> u64 {
        self.major
    }

    pub fn minor(&self) -> u64 {
        self.minor
    }

    pub fn patch(&self) -> u64 {
        self.patch
    }

    pub fn pre(&self) -> Option<&str> {
        if self.pre.is_empty() {
            None
        } else {
            Some(self.pre)
        }
    }

    pub fn build_code(&self) -> Option<&str> {
        if self.build_code.is_empty() {
            None
        } else {
            Some(self.build_code)
        }
    }
}

pub struct Release<'a> {
    raw: &'a str,
    package: &'a str,
    version_raw: &'a str,
    version: Option<Version<'a>>,
}

impl<'a> Release<'a> {
    pub fn new(release: &'a str) -> Release<'a> {
        if let Some(caps) = RELEASE_REGEX.captures(release) {
            Release {
                raw: release,
                package: caps.get(1).unwrap().as_str(),
                version_raw: caps.get(2).unwrap().as_str(),
                version: Version::new(caps.get(2).unwrap().as_str()),
            }
        } else {
            Release {
                raw: release,
                package: "",
                version_raw: release,
                version: None,
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

    pub fn version(&self) -> Option<&Version<'a>> {
        self.version.as_ref()
    }
}
