const fs = require("fs");
const { Release } = require("./lib/parser.js");

test("parse snapshots", () => {
  // check against the rust snapshots
  fs.readdirSync("tests/snapshots").forEach((snap) => {
    const match = fs
      .readFileSync(`tests/snapshots/${snap}`, "utf-8")
      .match(/expression: \"&release\((.*?)\)\"\n.*?---\n(.*)?$/s);
    const release = JSON.parse(JSON.parse(`"${match[1]}"`));
    const output = JSON.parse(match[2]);

    const parsedRelease = new Release(release);
    expect(parsedRelease.versionRaw).toEqual(output.version_raw);
    expect(parsedRelease.getBuildHash()).toEqual(output.build_hash);
    if (output.version_parsed) {
      const v = parsedRelease.versionParsed;
      expect(v.major).toEqual(output.version_parsed.major);
      expect(v.minor).toEqual(output.version_parsed.minor);
      expect(v.patch).toEqual(output.version_parsed.patch);
      expect(v.revision).toEqual(output.version_parsed.revision);
      expect(v.rawQuad).toEqual(output.version_parsed.raw_quad);
      expect(v.pre || null).toEqual(output.version_parsed.pre || null);
      expect(v.buildCode || null).toEqual(
        output.version_parsed.build_code || null
      );
    } else {
      expect(parsedRelease.versionParsed).toEqual(undefined);
    }
  });
});
