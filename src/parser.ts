const RELEASE_REGEX = /^(@?[^@]+)@(.*?)$/;
const VERSION_REGEX = new RegExp(
  `^
    (0|[1-9][0-9]*)
    (?:\\.(0|[1-9][0-9]*))
    (?:\\.(0|[1-9][0-9]*))?
    (?:-?
        ((?:0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*)
        (?:\\.(?:0|[1-9][0-9]*|[0-9]*[a-zA-Z-][0-9a-zA-Z-]*))*))?
    (?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?
$`.replace(/\s/g, "")
);
const HEX_REGEX = /^[a-fA-F0-9]+$/;
const VALID_RELEASE_REGEX = /^[^/\r\n]*$/;

export class InvalidRelease extends Error {
  code: string;

  constructor(code: string) {
    super(code);
    this.code = code;
  }
}

export interface Version {
  raw: string;
  major: number;
  minor: number;
  patch: number;
  pre?: string;
  buildCode?: string;
}

export class Release {
  raw: string;
  package?: string;
  versionRaw: string;
  versionParsed?: Version;

  constructor(release: string) {
    release = release.trim();
    if (release.length > 250) {
      throw new InvalidRelease("TOO_LONG");
    } else if (release == "." || release == ".." || release == "latest") {
      throw new InvalidRelease("RESTRICTED_NAME");
    } else if (!release.match(VALID_RELEASE_REGEX)) {
      throw new InvalidRelease("BAD_CHARACTERS");
    }
    const releaseMatch = release.match(RELEASE_REGEX);
    if (releaseMatch) {
      this.raw = release;
      this.package = releaseMatch[1];
      this.versionRaw = releaseMatch[2];
      this.versionParsed = parseVersion(this.versionRaw) || undefined;
    } else {
      this.raw = release;
      this.versionRaw = release;
      this.versionParsed = undefined;
    }
  }

  getBuildHash(): string | null {
    if (
      this.versionParsed &&
      this.versionParsed.buildCode &&
      isBuildHash(this.versionParsed.buildCode)
    ) {
      return this.versionParsed.buildCode;
    }
    if (isBuildHash(this.versionRaw)) {
      return this.versionRaw;
    }
    return null;
  }

  describe(): string {
    const hash = this.getBuildHash();
    const shortHash = hash && hash.substr(0, 12) || null;
    const v = this.versionParsed;
    let rv = "";
    if (v) {
      rv += `${v.major}.${v.minor}.${v.patch}`;
      if (v.pre) {
        rv += "-" + v.pre;
      }
      if (shortHash) {
        rv += ` (${shortHash})`;
      }
      else if (v.buildCode) {
        rv += ` (${v.buildCode})`;
      }
    } else if (shortHash) {
      rv = shortHash;
    } else {
      rv = this.versionRaw;
    }
    return rv;
  }
}

function isBuildHash(s: string): boolean {
  switch (s.length) {
    case 12:
    case 16:
    case 20:
    case 32:
    case 40:
    case 64:
      return !!s.match(HEX_REGEX);
    default:
      return false;
  }
}

function parseVersion(version: string): Version | null {
  const match = version.match(VERSION_REGEX);
  if (!match) {
    return null;
  }

  return {
    raw: version,
    major: parseInt(match[1], 10),
    minor: parseInt(match[2] || "0", 10),
    patch: parseInt(match[3] || "0", 10),
    pre: match[4] || undefined,
    buildCode: match[5] || undefined
  };
}
