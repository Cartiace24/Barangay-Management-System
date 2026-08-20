export interface SessionUser { id: number; fullName: string; username: string; roleId: number; roleName: string; permissions: string[]; }
export interface UserRecord { id:number; fullName:string; username:string; positionId:number|null; positionTitle:string|null; roleId:number|null; roleName:string|null; status:string; lastLoginAt:string|null; }
export interface Lookup { id:number; name:string; status:string; }
