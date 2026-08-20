import { invoke } from "@tauri-apps/api/core";
import type { Lookup, SessionUser, UserRecord } from "../types/auth";
export interface UserInput { fullName:string; username:string; password?:string; positionId?:number; roleId:number; status:string; }
export function login(username:string,password:string){return invoke<{status:string;user:SessionUser|null}>("login",{username,password});}
export function logout(){return invoke<void>("logout");}
export function getCurrentSession(){return invoke<SessionUser|null>("current_session");}
export function listUsers(){return invoke<UserRecord[]>("list_users");}
export function listRoles(){return invoke<Lookup[]>("list_roles");}
export function listPositions(){return invoke<Lookup[]>("list_positions");}
export function createPosition(title:string,status="active"){return invoke<Lookup>("create_position",{input:{title,status}});}
export function createUser(input:UserInput){return invoke<UserRecord>("create_user",{input});}
export function updateUser(userId:number,input:UserInput){return invoke<UserRecord>("update_user",{userId,input});}
export function resetUserPassword(userId:number,password:string){return invoke<void>("reset_user_password",{input:{userId,password}});}
export function changeOwnPassword(currentPassword:string,newPassword:string){return invoke<void>("change_own_password",{input:{currentPassword,newPassword}});}
