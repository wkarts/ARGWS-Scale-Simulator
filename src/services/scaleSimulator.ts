import { invokeCommand } from "./tauri";

export type ScaleSimulatorDriver = "wt1000n" | "wt27r" | "wt3000ir" | "toledo8540" | "toledo_prix3" | "a12e" | "lider_ld2052" | "filizola" | "generic_ascii";
export type ScaleSimulatorMode = "tcp_server" | "tcp_server_command" | "serial_continuous" | "serial_command";

export interface ScaleSimulatorConfig {
  id?: string | null;
  name: string;
  driver: ScaleSimulatorDriver;
  mode: ScaleSimulatorMode;
  host: string;
  port: number;
  serial_port: string;
  baud_rate: number;
  toledo8540_stop_bits?: 1 | 2;
  serial_data_bits?: 5 | 6 | 7 | 8;
  serial_parity?: "none" | "even" | "odd";
  serial_stop_bits?: 1 | 2;
  frame_override_enabled?: boolean;
  frame_prefix_hex?: string | null;
  frame_suffix_hex?: string | null;
  frame_checksum_mode?: "protocol" | "corrupt" | "omit";
  capacity_kg: number;
  division_kg: number;
  initial_weight_kg: number;
  variation_kg: number;
  interval_ms: number;
  stable: boolean;
  zero_tracking: boolean;
  unit: string;
}

export interface ScaleSimulatorPreset {
  key: string;
  name: string;
  description: string;
  driver: ScaleSimulatorDriver;
  default_port: number;
  default_serial_port: string;
  default_baud_rate: number;
  command_mode: boolean;
  serial_mode: boolean;
  sample_frame: string;
  adp_equipment: string;
}

export interface ScaleSimulatorStatus {
  id: string;
  name: string;
  driver: ScaleSimulatorDriver;
  mode: ScaleSimulatorMode;
  host: string;
  port: number;
  serial_port: string;
  baud_rate: number;
  serial_data_bits?: number;
  serial_parity?: string;
  serial_stop_bits?: number;
  frame_override_enabled?: boolean;
  running: boolean;
  clients: number;
  weight_kg: number;
  gross_kg: number;
  tare_kg: number;
  net_kg: number;
  stable: boolean;
  last_frame: string;
  started_at: string;
}

export interface ScaleSimulatorLogEntry {
  at: string;
  level: string;
  message: string;
}

export interface ScaleSimulatorSerialPortInfo {
  port_name: string;
  display_name: string;
  port_type: string;
  description?: string | null;
  manufacturer?: string | null;
  serial_number?: string | null;
  vid?: number | null;
  pid?: number | null;
  in_use_by_simulator: boolean;
}

export interface ScaleSimulatorActionResult {
  id: string;
  message: string;
  status?: ScaleSimulatorStatus | null;
}

export function defaultScaleConfig(): ScaleSimulatorConfig {
  return {
    id: null,
    name: "Balança WT1000N 01",
    driver: "wt1000n",
    mode: "serial_continuous",
    host: "127.0.0.1",
    port: 9101,
    serial_port: "COM1",
    baud_rate: 9600,
    serial_data_bits: 8,
    serial_parity: "none",
    serial_stop_bits: 1,
    frame_override_enabled: false,
    frame_prefix_hex: null,
    frame_suffix_hex: null,
    frame_checksum_mode: "protocol",
    capacity_kg: 80000,
    division_kg: 1,
    initial_weight_kg: 1234,
    variation_kg: 0,
    interval_ms: 500,
    stable: true,
    zero_tracking: true,
    unit: "kg",
  };
}

export function loadScaleSimulatorPresets() {
  return invokeCommand<ScaleSimulatorPreset[]>("scale_simulator_presets");
}

export function loadScaleSimulatorSerialPorts() {
  return invokeCommand<ScaleSimulatorSerialPortInfo[]>("scale_simulator_serial_ports");
}

export function previewScaleSimulatorFrame(config: ScaleSimulatorConfig) {
  return invokeCommand<string>("scale_simulator_preview_frame", { config });
}

export function startScaleSimulator(config: ScaleSimulatorConfig) {
  return invokeCommand<ScaleSimulatorActionResult>("scale_simulator_start", { config });
}

export function stopScaleSimulator(id: string) {
  return invokeCommand<ScaleSimulatorActionResult>("scale_simulator_stop", { id });
}

export function stopAllScaleSimulators() {
  return invokeCommand<ScaleSimulatorActionResult>("scale_simulator_stop_all");
}

export function listScaleSimulators() {
  return invokeCommand<ScaleSimulatorStatus[]>("scale_simulator_list");
}

export function loadScaleSimulatorLogs(id: string) {
  return invokeCommand<ScaleSimulatorLogEntry[]>("scale_simulator_logs", { id });
}

export function setScaleSimulatorWeight(id: string, weightKg: number) {
  return invokeCommand<ScaleSimulatorActionResult>("scale_simulator_set_weight", { id, weight_kg: weightKg });
}

export function executeScaleSimulatorAction(id: string, action: string, value?: number | null) {
  return invokeCommand<ScaleSimulatorActionResult>("scale_simulator_action", { id, action, value: value ?? null });
}
