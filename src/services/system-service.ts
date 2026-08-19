import { invoke } from "@tauri-apps/api/core";
import type { DatabaseStatus } from "../types/database";

/** Typed boundary for Tauri system services; UI components should use services, not SQL. */
export function getDatabaseStatus(): Promise<DatabaseStatus> {
  return invoke<DatabaseStatus>("get_database_status");
}
