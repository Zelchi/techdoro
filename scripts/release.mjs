import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";
import path from "node:path";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const FORMATS = {
  "linux-x86_64": ["deb", "rpm"],
  "windows-x86_64": ["exe"],
};

function isFile(file) {
  try {
    return statSync(file).isFile();
  } catch {
    return false;
  }
}

export function version(root = ROOT) {
  const cargoToml = readFileSync(path.join(root, "Cargo.toml"), "utf8");
  let inPackageSection = false;

  for (const line of cargoToml.split(/\r?\n/)) {
    if (/^\s*\[[^\]]+\]\s*$/.test(line)) {
      inPackageSection = line.trim() === "[package]";
      continue;
    }
    if (inPackageSection) {
      const match = line.match(/^\s*version\s*=\s*"([^"]+)"\s*(?:#.*)?$/);
      if (match) return match[1];
    }
  }

  throw new Error("Could not find [package].version in Cargo.toml");
}

export function validateTag(tag, root = ROOT) {
  if (tag !== `v${version(root)}`) {
    throw new Error("Release tag must match Cargo.toml");
  }
  const notes = path.join(root, ".github", "release-notes", `${tag}.md`);
  if (!isFile(notes) || !readFileSync(notes, "utf8").trim()) {
    throw new Error("Release notes are missing or empty");
  }
}

export function stage(source, destination, platform, verifier) {
  const formats = FORMATS[platform];
  if (!formats) throw new Error(`Unsupported platform: ${platform}`);

  mkdirSync(destination, { recursive: true });
  const entries = {};

  for (const extension of formats) {
    const installers = readdirSync(source, { withFileTypes: true })
      .filter((entry) => entry.isFile() && path.extname(entry.name) === `.${extension}`)
      .map((entry) => path.join(source, entry.name));
    if (installers.length !== 1) {
      throw new Error(`Expected one .${extension} installer, found ${installers.length}`);
    }

    const installer = installers[0];
    const signaturePath = `${installer}.sig`;
    if (!isFile(signaturePath)) throw new Error(`Missing signature: ${signaturePath}`);
    const signature = readFileSync(signaturePath, "utf8").trim();
    if (!signature) throw new Error(`Missing signature: ${signaturePath}`);

    if (verifier) {
      const result = spawnSync(verifier, ["--verify-update", installer, signaturePath], {
        stdio: "inherit",
      });
      if (result.error) throw result.error;
      if (result.status !== 0) {
        throw new Error(`Signature verification failed for ${path.basename(installer)}`);
      }
    }

    for (const file of [installer, signaturePath]) {
      const target = path.join(destination, path.basename(file));
      if (existsSync(target)) throw new Error(`Duplicate artifact name: ${path.basename(file)}`);
      copyFileSync(file, target);
    }

    entries[extension] = {
      filename: path.basename(installer),
      signature,
      format: extension === "exe" ? "nsis" : extension,
    };
  }

  writeFileSync(
    path.join(destination, `${platform}.json`),
    `${JSON.stringify(entries, null, 2)}\n`,
  );
}

function encodePathSegment(value) {
  return encodeURIComponent(value).replace(/[!'()*]/g, (character) =>
    `%${character.charCodeAt(0).toString(16).toUpperCase()}`,
  );
}

export function manifest(directory, repository, tag, root = ROOT) {
  validateTag(tag, root);
  const platforms = {};
  const packages = {};

  for (const [platform, formats] of Object.entries(FORMATS)) {
    const entries = JSON.parse(
      readFileSync(path.join(directory, `${platform}.json`), "utf8"),
    );

    for (const extension of formats) {
      const entry = entries[extension];
      if (!entry || typeof entry.filename !== "string") {
        throw new Error(`Missing .${extension} entry for ${platform}`);
      }
      const { filename, ...metadata } = entry;
      if (
        !filename ||
        filename === "." ||
        filename === ".." ||
        path.basename(filename) !== filename ||
        filename.includes("\\") ||
        !isFile(path.join(directory, filename))
      ) {
        throw new Error(`Missing installer: ${filename}`);
      }

      const signaturePath = path.join(directory, `${filename}.sig`);
      if (
        !isFile(signaturePath) ||
        readFileSync(signaturePath, "utf8").trim() !== metadata.signature
      ) {
        throw new Error(`Signature mismatch: ${filename}`);
      }

      const releaseUrl = `https://github.com/${repository}/releases/download/${encodePathSegment(tag)}/${encodePathSegment(filename)}`;
      const artifact = { ...metadata, url: releaseUrl };
      if (extension === "exe") {
        platforms[platform] = artifact;
      } else {
        packages[platform] ??= {};
        packages[platform][extension] = artifact;
      }
    }
  }

  const notes = readFileSync(
    path.join(root, ".github", "release-notes", `${tag}.md`),
    "utf8",
  );
  const result = {
    version: version(root),
    notes,
    pub_date: new Date().toISOString(),
    platforms,
    packages,
  };
  writeFileSync(path.join(directory, "native-latest.json"), `${JSON.stringify(result, null, 2)}\n`);
}

function parseOptions(args, allowedFlags) {
  const positional = [];
  const options = {};
  for (let index = 0; index < args.length; index += 1) {
    const argument = args[index];
    if (!argument.startsWith("--")) {
      positional.push(argument);
      continue;
    }
    if (!allowedFlags.includes(argument)) throw new Error(`Unknown option: ${argument}`);
    const value = args[index + 1];
    if (!value || value.startsWith("--")) throw new Error(`Expected a value after ${argument}`);
    options[argument.slice(2)] = value;
    index += 1;
  }
  return { positional, options };
}

function resolvePath(value) {
  return path.resolve(process.cwd(), value);
}

export function main(args = process.argv.slice(2)) {
  const [command, ...commandArgs] = args;
  if (command === "validate-tag") {
    const { positional } = parseOptions(commandArgs, []);
    if (positional.length !== 1) throw new Error("Usage: release.mjs validate-tag TAG");
    validateTag(positional[0]);
    return;
  }

  if (command === "stage") {
    const { positional, options } = parseOptions(commandArgs, ["--verify", "--source", "--destination"]);
    if (positional.length !== 1) {
      throw new Error("Usage: release.mjs stage PLATFORM [--verify BINARY] [--source DIR] [--destination DIR]");
    }
    stage(
      resolvePath(options.source ?? "target/packages"),
      resolvePath(options.destination ?? "target/release-assets"),
      positional[0],
      options.verify,
    );
    return;
  }

  if (command === "manifest") {
    const { positional, options } = parseOptions(commandArgs, ["--directory"]);
    if (positional.length !== 2) {
      throw new Error("Usage: release.mjs manifest REPOSITORY TAG [--directory DIR]");
    }
    manifest(
      resolvePath(options.directory ?? "release-assets"),
      positional[0],
      positional[1],
    );
    return;
  }

  throw new Error("Usage: release.mjs {validate-tag|stage|manifest} ...");
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    console.error(`release: ${error.message}`);
    process.exitCode = 1;
  }
}
