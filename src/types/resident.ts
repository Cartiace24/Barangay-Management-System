export type ResidentStatus = "active" | "inactive" | "deceased" | "moved_out";
export type ResidentSex = "male" | "female" | "other" | "unspecified";

export interface Resident {
  id: number;
  residentCode: string;
  firstName: string;
  middleName: string | null;
  lastName: string;
  suffix: string | null;
  birthDate: string | null;
  sex: ResidentSex;
  civilStatus: string | null;
  nationality: string;
  contactNumber: string | null;
  email: string | null;
  address: string | null;
  barangay: string | null;
  municipality: string | null;
  province: string | null;
  occupation: string | null;
  status: ResidentStatus;
  registeredAt: string;
  createdAt: string;
  updatedAt: string;
}

export interface ResidentInput {
  firstName: string;
  middleName?: string;
  lastName: string;
  suffix?: string;
  birthDate?: string;
  sex: ResidentSex;
  civilStatus?: string;
  nationality?: string;
  contactNumber?: string;
  email?: string;
  address?: string;
  barangay?: string;
  municipality?: string;
  province?: string;
  occupation?: string;
  status: ResidentStatus;
}

export interface ResidentListQuery {
  search?: string;
  status?: ResidentStatus | "all";
  sex?: ResidentSex | "all";
  page?: number;
  pageSize?: number;
  sortBy?: "lastName" | "firstName" | "registeredAt" | "residentCode";
  sortDirection?: "asc" | "desc";
}

export interface ResidentListResult { records: Resident[]; total: number; }
export interface HouseholdSummary { householdCode: string; addressLine: string; purok: string | null; relationshipToHead: string; isHouseholdHead: boolean; }
export interface DocumentSummary { documentNumber: string; documentType: string; status: string; issuedAt: string; }
export interface ActivitySummary { action: string; createdAt: string; userName: string | null; }
export interface ResidentProfile { resident: Resident; household: HouseholdSummary | null; documents: DocumentSummary[]; activity: ActivitySummary[]; }
