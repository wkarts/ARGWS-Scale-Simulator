/** Display the UTF-8 bytes sent by Rust's String::as_bytes(), including controls. */
export function frameHex(frame: string): string {
  if (!frame) return "—";
  return Array.from(new TextEncoder().encode(frame), byte => byte.toString(16).padStart(2, "0").toUpperCase()).join(" ");
}
