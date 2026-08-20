import { invoke } from "@tauri-apps/api/core";
import type { Resident, ResidentInput, ResidentListQuery, ResidentListResult, ResidentProfile } from "../types/resident";

export const listResidents = (query: ResidentListQuery): Promise<ResidentListResult> => invoke("list_residents", { query });
export const getResidentProfile = (residentId: number): Promise<ResidentProfile> => invoke("get_resident_profile", { residentId });
export const createResident = (input: ResidentInput): Promise<Resident> => invoke("create_resident", { input });
export const updateResident = (residentId: number, input: ResidentInput): Promise<Resident> => invoke("update_resident", { residentId, input });
export const archiveResident = (residentId: number): Promise<void> => invoke("archive_resident", { residentId });
