import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { manifest, stage, validateTag, version } from "./release.mjs";

function temporaryDirectory(t) {
  const directory = mkdtempSync(path.join(os.tmpdir(), "techdoro-release-test-"));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  return directory;
}

test("manifest matches signed installers and separates formats", (t) => {
  const root = temporaryDirectory(t);
  const assets = path.join(root, "assets");
  const artifacts = [
    ["linux-x86_64", ["techdoro.deb", "techdoro.rpm"]],
    ["windows-x86_64", ["Techdoro setup.exe"]],
  ];

  for (const [platform, filenames] of artifacts) {
    const source = path.join(root, platform);
    mkdirSync(source);
    for (const filename of filenames) {
      writeFileSync(path.join(source, filename), "installer");
      writeFileSync(path.join(source, `${filename}.sig`), "signed\n");
    }
    stage(source, assets, platform);
  }

  const tag = `v${version()}`;
  manifest(assets, "Zelchi/Techdoro", tag);
  const result = JSON.parse(readFileSync(path.join(assets, "native-latest.json"), "utf8"));
  assert.equal(result.packages["linux-x86_64"].deb.format, "deb");
  assert.equal(result.packages["linux-x86_64"].rpm.format, "rpm");
  assert.equal("linux-x86_64" in result.platforms, false);
  assert.equal(result.platforms["windows-x86_64"].format, "nsis");
  assert.match(result.platforms["windows-x86_64"].url, /%20/);
  assert.match(result.pub_date, /^\d{4}-\d\d-\d\dT.*Z$/);
});

test("staging rejects missing or ambiguous installers", (t) => {
  const root = temporaryDirectory(t);
  const destination = path.join(root, "assets");
  assert.throws(() => stage(root, destination, "linux-x86_64"), /Expected one \.deb installer, found 0/);

  writeFileSync(path.join(root, "one.deb"), "installer");
  writeFileSync(path.join(root, "two.deb"), "installer");
  assert.throws(() => stage(root, destination, "linux-x86_64"), /Expected one \.deb installer, found 2/);
});

test("release tags must match the package version and have notes", () => {
  assert.throws(() => validateTag("v0.0.0"), /Release tag must match Cargo.toml/);
  assert.doesNotThrow(() => validateTag(`v${version()}`));
});
