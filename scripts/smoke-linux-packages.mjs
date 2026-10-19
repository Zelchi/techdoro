import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readdirSync } from "node:fs";
import path from "node:path";
import { version } from "./release.mjs";

const packageDirectory = path.resolve("target/packages");
const expectedVersion = `Techdoro ${version()}`;

function packageFile(extension) {
  const matches = readdirSync(packageDirectory).filter((name) => name.endsWith(extension));
  assert.equal(matches.length, 1, `Expected one ${extension} package, found ${matches.length}`);
  return matches[0];
}

function run(command, args) {
  execFileSync(command, args, { stdio: "inherit" });
}

function verifyInstalledVersion(command, args = []) {
  const output = execFileSync(command, args, { encoding: "utf8" }).trim();
  assert.equal(output, expectedVersion);
  console.log(`Verified installed ${expectedVersion}: ${command}`);
}

if (process.platform !== "linux") {
  throw new Error("Linux package smoke tests must run on Linux");
}

const deb = packageFile(".deb");
const rpm = packageFile(".rpm");
run("sudo", ["apt-get", "install", "-y", `./target/packages/${deb}`]);
verifyInstalledVersion("techdoro", ["--version"]);

const packageVolume = `${packageDirectory}:/packages:ro`;
run("docker", [
  "run",
  "--rm",
  "--volume",
  packageVolume,
  "debian:trixie-slim",
  "sh",
  "-ec",
  "apt-get update && cd /packages && apt-get install -y \"./$1\" && actual=\"$(techdoro --version)\" && test \"$actual\" = \"$2\" && printf '%s\\n' \"$actual\"",
  "smoke",
  deb,
  expectedVersion,
]);

run("docker", [
  "run",
  "--rm",
  "--volume",
  packageVolume,
  "fedora:latest",
  "sh",
  "-ec",
  "dnf install -y \"/packages/$1\" && actual=\"$(techdoro --version)\" && test \"$actual\" = \"$2\" && printf '%s\\n' \"$actual\"",
  "smoke",
  rpm,
  expectedVersion,
]);
