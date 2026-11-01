import { spawnSync } from 'node:child_process';
export function validateCargoMetadata(cwd, execute = spawnSync) {
  // Metadata can exceed Node's default 1 MiB capture limit with Tauri targets.
  // Cargo still resolves/checks the complete locked graph; only unused stdout
  // is discarded. Never replace this with --no-deps or ignore a nonzero status.
  const result = execute(process.env.CARGO || 'cargo', ['metadata', '--manifest-path', 'src-tauri/Cargo.toml', '--locked', '--format-version', '1'], {
    cwd, encoding: 'utf8', stdio: ['ignore', 'ignore', 'pipe'], maxBuffer: 16 * 1024 * 1024, timeout: 180000,
  });
  if (result.status !== 0 || result.error) {
    const reason = [result.error?.code, result.error?.message, `status=${result.status}`, `signal=${result.signal}`].filter(Boolean).join('; ');
    throw new Error(`Cargo metadata validation failed (${reason}): ${String(result.stderr || '').slice(-8000)}`);
  }
}
