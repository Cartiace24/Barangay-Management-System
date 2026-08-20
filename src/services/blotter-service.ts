import { invoke } from "@tauri-apps/api/core";
import type { BlotterCase } from "../types/blotter";
export const listBlotterCases=(query:Record<string,unknown>):Promise<{records:BlotterCase[];total:number}>=>invoke("list_blotter_cases",{query});
export const createBlotterCase=(input:Record<string,unknown>):Promise<BlotterCase>=>invoke("create_blotter_case",{input});
export const updateBlotterCase=(caseId:number,input:Record<string,unknown>):Promise<BlotterCase>=>invoke("update_blotter_case",{caseId,input});
