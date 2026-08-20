import { Eye, EyeOff } from "lucide-react";
import { useState } from "react";
import { Button } from "../components/ui/button";
import { Checkbox } from "../components/ui/checkbox";
import { FormField } from "../components/ui/form-field";
import { Input } from "../components/ui/input";
import { login } from "../services/auth-service";
import type { SessionUser } from "../types/auth";
import type { BarangayProfileSummary } from "../types/barangay";
import { BarangayLogo } from "../components/shared/BarangayLogo";

export function LoginPage({ profile, onAuthenticated }: { profile: BarangayProfileSummary; onAuthenticated: (user: SessionUser) => void }) {
  const [passwordVisible, setPasswordVisible] = useState(false);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const submit = async () => { setSubmitting(true); setError(""); try { const result=await login(username,password); if(result.status==="success"&&result.user) onAuthenticated(result.user); else setError(result.status==="disabled"?"This account has been disabled. Contact an administrator.":"Invalid username or password."); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setSubmitting(false); } };

  return <main className="grid min-h-screen place-items-center bg-background p-8"><section className="w-full max-w-md overflow-hidden rounded-lg border border-border bg-white"><div className="border-b border-border bg-primary px-8 py-7 text-center text-white"><div className="mx-auto grid size-11 place-items-center rounded-md border border-white/35 bg-white/10"><BarangayLogo /></div><h1 className="mt-4 text-lg font-bold">Barangay Management System</h1><p className="mt-2 text-sm font-semibold">{profile.name}</p><p className="text-xs text-blue-100">{profile.municipality}, {profile.province}</p></div><form className="space-y-5 p-8" onSubmit={(event) => { event.preventDefault(); void submit(); }}><div><h2 className="text-base font-bold">Sign in</h2><p className="mt-1 text-sm text-muted-foreground">Enter your assigned account credentials.</p></div><FormField label="Username" htmlFor="username"><Input id="username" autoComplete="username" placeholder="Enter username" value={username} onChange={(event) => setUsername(event.target.value)} /></FormField><FormField label="Password" htmlFor="password"><div className="relative"><Input id="password" type={passwordVisible ? "text" : "password"} autoComplete="current-password" placeholder="Enter password" className="pr-10" value={password} onChange={(event) => setPassword(event.target.value)} /><button type="button" aria-label={passwordVisible ? "Hide password" : "Show password"} onClick={() => setPasswordVisible((visible) => !visible)} className="absolute right-2 top-1/2 -translate-y-1/2 rounded p-1 text-muted-foreground">{passwordVisible ? <EyeOff className="size-4" /> : <Eye className="size-4" />}</button></div></FormField>{error && <p className="text-sm text-destructive">{error}</p>}<label className="flex items-center gap-2 text-sm text-muted-foreground"><Checkbox />Remember this session on this computer</label><Button type="submit" className="w-full" size="lg" disabled={submitting}>{submitting ? "Signing in..." : "Sign in"}</Button><p className="text-center text-xs text-muted-foreground">For authorized barangay personnel only.</p></form></section></main>;
}
