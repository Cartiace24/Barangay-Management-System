export interface Household { id: number; householdCode: string; addressLine: string; purok: string | null; status: "active" | "inactive"; registeredAt: string; householdHead: string | null; memberCount: number; }
export interface HouseholdInput { addressLine: string; purok?: string; status: "active" | "inactive"; headResidentId?: number; }
export interface HouseholdListQuery { search?: string; status?: "active" | "inactive" | "all"; page?: number; pageSize?: number; }
export interface HouseholdListResult { records: Household[]; total: number; }
export interface ResidentOption { id: number; residentCode: string; fullName: string; }
export interface HouseholdMember { residentId: number; residentCode: string; fullName: string; relationshipToHead: string; isHouseholdHead: boolean; status: string; }
export interface HouseholdActivity { action: string; createdAt: string; userName: string | null; }
export interface HouseholdProfile { household: Household; members: HouseholdMember[]; activity: HouseholdActivity[]; }
