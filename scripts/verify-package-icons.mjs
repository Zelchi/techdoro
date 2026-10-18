import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

// Inspect actual PE resources: an icon next to an EXE does not embed it.
function verifyWindowsIcon(path) {
  const pe = readFileSync(path);
  assert.equal(pe.toString('ascii', 0, 2), 'MZ', path);
  const header = pe.readUInt32LE(0x3c);
  assert.equal(pe.readUInt32LE(header), 0x4550, path);
  const optional = header + 24;
  const magic = pe.readUInt16LE(optional);
  assert.ok(magic === 0x20b || magic === 0x10b, path);
  const directory = optional + (magic === 0x20b ? 112 : 96);
  const resourceRva = pe.readUInt32LE(directory + 16);
  assert.ok(resourceRva, `${path}: missing resource directory`);
  const sections = optional + pe.readUInt16LE(header + 20);
  let resource;
  for (let i = 0; i < pe.readUInt16LE(header + 6); i++) {
    const section = sections + i * 40;
    const address = pe.readUInt32LE(section + 12);
    const size = Math.max(pe.readUInt32LE(section + 8), pe.readUInt32LE(section + 16));
    if (resourceRva >= address && resourceRva < address + size) {
      resource = pe.readUInt32LE(section + 20) + resourceRva - address;
      break;
    }
  }
  assert.notEqual(resource, undefined, `${path}: invalid resource section`);
  const count = pe.readUInt16LE(resource + 12) + pe.readUInt16LE(resource + 14);
  const types = Array.from({ length: count }, (_, i) => pe.readUInt32LE(resource + 16 + i * 8));
  assert.ok(types.includes(3) && types.includes(14), `${path}: missing ICON/GROUP_ICON resources`);
  console.log(`Verified embedded Windows icon: ${path}`);
}

function verifyLinuxPackage(path) {
  const root = mkdtempSync(join(tmpdir(), 'techdoro-package-icons-'));
  try {
    if (path.endsWith('.deb')) {
      execFileSync('dpkg-deb', ['--extract', path, root], { stdio: 'inherit' });
    } else {
      execFileSync('bsdtar', ['--extract', '--file', path, '--directory', root], {
        stdio: 'inherit',
      });
    }
    const applications = join(root, 'usr/share/applications');
    assert.deepEqual(readdirSync(applications), ['com.zelchi.techdoro.desktop']);
    const desktop = join(applications, 'com.zelchi.techdoro.desktop');
    execFileSync('desktop-file-validate', [desktop], { stdio: 'inherit' });
    const entry = readFileSync(desktop, 'utf8');
    for (const value of ['Exec=techdoro', 'Icon=techdoro', 'StartupWMClass=com.zelchi.techdoro',
      'X-GNOME-UsesNotifications=true']) {
      assert.ok(entry.split(/\r?\n/).includes(value), `${path}: missing ${value}`);
    }
    const icon = readFileSync(join(root, 'usr/share/icons/hicolor/256x256/apps/techdoro.png'));
    assert.ok(icon.equals(readFileSync('assets/linux/256x256.png')), `${path}: wrong icon`);
    assert.equal(icon.readUInt32BE(16), 256);
    assert.equal(icon.readUInt32BE(20), 256);
    assert.ok(readFileSync(join(root, 'usr/bin/techdoro')).equals(readFileSync('target/release/techdoro')),
      `${path}: stale application binary`);
    console.log(`Verified Linux launcher, icon and executable: ${path}`);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

const packages = readdirSync('target/packages');
if (process.platform === 'win32') {
  verifyWindowsIcon('target/release/techdoro.exe');
  const installers = packages.filter(name => name.endsWith('.exe'));
  assert.equal(installers.length, 1, 'Expected one Windows installer');
  for (const name of installers) verifyWindowsIcon(join('target/packages', name));
} else {
  for (const extension of ['.deb', '.rpm']) {
    const matches = packages.filter(name => name.endsWith(extension));
    assert.equal(matches.length, 1, `Expected one ${extension} installer`);
    verifyLinuxPackage(join('target/packages', matches[0]));
  }
}
