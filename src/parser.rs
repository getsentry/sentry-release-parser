use std::fmt;

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
    static ref DOTTED_BUILD_CODE_REGEX: Regex = Regex::new(
        r#"(?x)
        ^
            (?P<major>0|[1-9][0-9]*)
            (?:\.(?P<minor>0|[1-9][0-9]*))?
            (?:\.(?P<patch>0|[1-9][0-9]*))?
        $
    "#
    )
    .unwrap();
    static ref HEX_REGEX: Regex = Regex::new(r#"^[a-fA-F0-9]+$"#).unwrap();
}

#[derive(Debug, Clone)]
pub struct InvalidVersion;

impl std::error::Error for InvalidVersion {}

impl fmt::Display for InvalidVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid version")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Version<'a> {
    raw: &'a str,
    major: u64,
    minor: u64,
    patch: u64,
    pre: &'a str,
    build_code: &'a str,
}

fn is_build_hash(s: &str) -> bool {
    match s.len() {
        12 | 16 | 20 | 32 | 40 | 64 => HEX_REGEX.is_match(s),
        _ => false,
    }
}

impl<'a> Version<'a> {
    /// Parses a version from a string.
    pub fn parse(version: &'a str) -> Result<Version<'a>, InvalidVersion> {
        let caps = if let Some(caps) = VERSION_REGEX.captures(version) {
            caps
        } else {
            return Err(InvalidVersion);
        };
        Ok(Version {
            raw: version,
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

    /// Returns the major version component.
    pub fn major(&self) -> u64 {
        self.major
    }

    /// Returns the minor version component.
    pub fn minor(&self) -> u64 {
        self.minor
    }

    /// Returns the patch level version component.
    pub fn patch(&self) -> u64 {
        self.patch
    }

    /// If a pre-release identifier is included returns that.
    pub fn pre(&self) -> Option<&str> {
        if self.pre.is_empty() {
            None
        } else {
            Some(self.pre)
        }
    }

    /// If a build code is included returns that.
    pub fn build_code(&self) -> Option<&str> {
        if self.build_code.is_empty() {
            None
        } else {
            Some(self.build_code)
        }
    }

    /// Returns an internally normalized build code.
    ///
    /// This value is useful for ordering but it should never be shown to a user
    /// as it might be very confusing.  For instance if a build code looks like a
    /// dotted version it ends up being padded to 32 characters.
    pub fn normalized_build_code(&self) -> String {
        if let Some(caps) = DOTTED_BUILD_CODE_REGEX.captures(self.build_code) {
            format!(
                "{:012}{:010}{:010}",
                caps[1].parse::<u64>().unwrap_or(0),
                caps.get(2)
                    .and_then(|x| x.as_str().parse::<u64>().ok())
                    .unwrap_or(0),
                caps.get(3)
                    .and_then(|x| x.as_str().parse::<u64>().ok())
                    .unwrap_or(0),
            )
        } else {
            self.build_code.to_ascii_lowercase()
        }
    }

    /// Returns the raw version as string.
    ///
    /// It's generally better to use `to_string` which normalizes.
    pub fn raw(&self) -> &str {
        self.raw
    }

    /// Returns the version triple (major, minor, patch)
    pub fn triple(&self) -> (u64, u64, u64) {
        (self.major, self.minor, self.patch)
    }

    /// Returns the version triple with an added pre-release marker.
    pub fn quad(&self) -> (u64, u64, u64, Option<&str>) {
        (self.major, self.minor, self.patch, self.pre())
    }
}

impl<'a> fmt::Display for Version<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())?;
        if let Some(pre) = self.pre() {
            write!(f, "-{}", pre)?;
        }
        if let Some(build_code) = self.build_code() {
            write!(f, "+{}", build_code)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReleaseType {
    Unqualified,
    Qualified,
    Versioned,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Release<'a> {
    raw: &'a str,
    package: &'a str,
    version_raw: &'a str,
    version: Option<Version<'a>>,
    ty: ReleaseType,
}

impl<'a> Release<'a> {
    /// Parses a release from a string.
    pub fn parse(release: &'a str) -> Release<'a> {
        if let Some(caps) = RELEASE_REGEX.captures(release) {
            let (version, ty) = if let Ok(version) = Version::parse(caps.get(2).unwrap().as_str()) {
                (Some(version), ReleaseType::Versioned)
            } else {
                (None, ReleaseType::Qualified)
            };
            Release {
                raw: release,
                package: caps.get(1).unwrap().as_str(),
                version_raw: caps.get(2).unwrap().as_str(),
                version,
                ty,
            }
        } else {
            Release {
                raw: release,
                package: "",
                version_raw: release,
                version: None,
                ty: ReleaseType::Unqualified,
            }
        }
    }

    /// Returns the raw version.
    ///
    /// It's generally better to use `to_string` which normalizes.
    pub fn raw(&self) -> &str {
        self.raw
    }

    /// Returns the contained package information.
    pub fn package(&self) -> Option<&str> {
        if self.package.is_empty() {
            None
        } else {
            Some(self.package)
        }
    }

    pub fn version_raw(&self) -> &str {
        self.version_raw
    }

    /// If a parsed version if available returns it.
    pub fn version(&self) -> Option<&Version<'a>> {
        self.version.as_ref()
    }

    /// Returns the build hash if available.
    pub fn build_hash(&self) -> Option<&str> {
        self.version
            .as_ref()
            .and_then(|x| x.build_code())
            .filter(|x| is_build_hash(x))
            .or_else(|| {
                if is_build_hash(self.version_raw()) {
                    Some(self.version_raw())
                } else {
                    None
                }
            })
    }

    /// Returns a short description
    pub fn describe(&self) -> ReleaseDescription<'_> {
        ReleaseDescription(self)
    }
}

#[derive(Debug)]
pub struct ReleaseDescription<'a>(&'a Release<'a>);

impl<'a> fmt::Display for ReleaseDescription<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ver) = self.0.version() {
            write!(f, "{}.{}.{}", ver.major(), ver.minor(), ver.patch())?;
            if let Some(pre) = ver.pre() {
                write!(f, "-{}", pre)?;
            }
            if let Some(build_code) = ver.build_code() {
                write!(f, " ({})", build_code)?;
            }
        } else if let Some(hash) = self.0.build_hash() {
            write!(f, "{}", hash.get(..12).unwrap_or(hash))?;
        } else {
            write!(f, "{}", self.0)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Release<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut have_package = false;
        if let Some(package) = self.package() {
            write!(f, "{}", package)?;
            have_package = true;
        }
        if let Some(version) = self.version() {
            if have_package {
                write!(f, "@")?;
            }
            write!(f, "{}", version)?;
        } else {
            if have_package {
                write!(f, "@")?;
            }
            write!(f, "{}", self.version_raw)?;
        }
        Ok(())
    }
}
