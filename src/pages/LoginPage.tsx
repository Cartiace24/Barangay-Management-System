import { Eye, EyeOff, Landmark } from "lucide-react";
import { useState } from "react";
import { Button } from "../components/ui/button";
import { Checkbox } from "../components/ui/checkbox";
import { FormField } from "../components/ui/form-field";
import { Input } from "../components/ui/input";
import { developmentBarangayProfile } from "../lib/development-profile";

/** Presentation-only login screen. Authentication will be connected later. */
export function LoginPage() {
  const [passwordVisible, setPasswordVisible] = useState(false);

  return <main className="grid min-h-screen place-items-center bg-background p-8"><section className="w-full max-w-md overflow-hidden rounded-lg border border-border bg-white"><div className="border-b border-border bg-primary px-8 py-7 text-center text-white"><div className="mx-auto grid size-11 place-items-center rounded-md border border-white/35 bg-white/10"><Landmark className="size-5" /></div><h1 className="mt-4 text-lg font-bold">Barangay Management System</h1><p className="mt-2 text-sm font-semibold">{developmentBarangayProfile.name}</p><p className="text-xs text-blue-100">{developmentBarangayProfile.municipality}, {developmentBarangayProfile.province}</p></div><form className="space-y-5 p-8" onSubmit={(event) => event.preventDefault()}><div><h2 className="text-base font-bold">Sign in</h2><p className="mt-1 text-sm text-muted-foreground">Enter your assigned account credentials.</p></div><FormField label="Username" htmlFor="username"><Input id="username" autoComplete="username" placeholder="Enter username" /></FormField><FormField label="Password" htmlFor="password"><div className="relative"><Input id="password" type={passwordVisible ? "text" : "password"} autoComplete="current-password" placeholder="Enter password" className="pr-10" /><button type="button" aria-label={passwordVisible ? "Hide password" : "Show password"} onClick={() => setPasswordVisible((visible) => !visible)} className="absolute right-2 top-1/2 -translate-y-1/2 rounded p-1 text-muted-foreground hover:bg-muted hover:text-foreground">{passwordVisible ? <EyeOff className="size-4" /> : <Eye className="size-4" />}</button></div></FormField><label className="flex items-center gap-2 text-sm text-muted-foreground"><Checkbox />Remember this session on this computer</label><Button type="submit" className="w-full" size="lg">Sign in</Button><p className="text-center text-xs text-muted-foreground">For authorized barangay personnel only.</p></form></section></main>;
}
