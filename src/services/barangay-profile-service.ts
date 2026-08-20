import { invoke } from "@tauri-apps/api/core";
import type { BarangayProfileSummary } from "../types/barangay";

export interface ProfileInput { name: string; municipality: string; province: string; address?: string; contactNumber?: string; email?: string; authorizedSignatory?: string; signatoryPosition?: string; }
export interface SetupInput { profile: ProfileInput; officials: { title: string; fullName: string }[]; administratorName: string; username: string; password: string; }

export function getBarangayProfile() { return invoke<BarangayProfileSummary | null>("get_barangay_profile"); }
export function authenticateUser(username: string, password: string) { return invoke<boolean>("authenticate_user", { input: { username, password } }); }
export function completeFirstTimeSetup(input: SetupInput) { return invoke<BarangayProfileSummary>("complete_first_time_setup", { input }); }
export function updateBarangayProfile(input: ProfileInput) { return invoke<BarangayProfileSummary>("update_barangay_profile", { input }); }
export function updateBrandingImage(kind: "logo" | "signature", image: { fileName: string; mimeType: string; bytes: number[] }) { return invoke<BarangayProfileSummary>("update_branding_image", { kind, image }); }
export function removeBrandingImage(kind: "logo" | "signature") { return invoke<BarangayProfileSummary>("remove_branding_image", { kind }); }
export async function getBrandingImage(kind: "logo" | "signature") { const data = await invoke<string | null>("get_branding_image", { kind }); return data ? `data:image/*;base64,${data}` : null; }
