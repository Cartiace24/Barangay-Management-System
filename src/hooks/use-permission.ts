import type { SessionUser } from "../types/auth";
export function hasPermission(user: SessionUser | null, permission: string) { return Boolean(user?.permissions.includes(permission)); }
