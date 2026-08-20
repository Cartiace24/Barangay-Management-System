import { Building2, FileText, Home, Landmark, Settings, ShieldAlert, Users, UserRoundCog, WalletCards } from "lucide-react";
import { BarangayLogo } from "./BarangayLogo";
import type { NavigationItem } from "../../types/navigation";
import { cn } from "../../lib/utils";
import type { BarangayProfileSummary } from "../../types/barangay";
import type { SessionUser } from "../../types/auth";

const navigationItems: NavigationItem[] = [
  { label: "Dashboard", icon: Home }, { label: "Residents", icon: Users, permission: "residents.view" }, { label: "Households", icon: Building2, permission: "households.view" }, { label: "Documents", icon: FileText, permission: "documents.view" }, { label: "Blotter", icon: ShieldAlert, permission: "blotter.view" }, { label: "Payments", icon: WalletCards, permission: "payments.view" }, { label: "Reports", icon: Landmark, permission: "reports.view" }, { label: "Officials", icon: UserRoundCog, permission: "users.view" }, { label: "Settings", icon: Settings, permission: "settings.view" },
];

interface SidebarProps { profile: BarangayProfileSummary; user: SessionUser; activeItem: string; onNavigate: (item: string) => void; }
export function Sidebar({ profile, user, activeItem, onNavigate }: SidebarProps) {
  const visibleItems = navigationItems.filter((item) => !item.permission || user.permissions.includes(item.permission));
  return <aside className="flex h-screen w-64 shrink-0 flex-col border-r border-[#102e54] bg-primary text-white"><div className="border-b border-white/15 px-5 py-5"><div className="flex items-center gap-3"><div className="grid size-10 place-items-center rounded-md border border-white/35 bg-white/10"><BarangayLogo key={profile.logoPath ?? "no-logo"} /></div><div className="min-w-0"><p className="text-xs font-bold leading-4 tracking-wide">{profile.name}</p><p className="mt-0.5 text-xs leading-4 text-blue-100">{profile.municipality}, {profile.province}</p></div></div></div><nav className="flex-1 px-3 py-4" aria-label="Main navigation"><p className="mb-2 px-2 text-[11px] font-bold tracking-[0.12em] text-blue-200">MAIN MENU</p><div className="space-y-1">{visibleItems.map(({ label, icon: Icon }) => <button key={label} type="button" onClick={() => onNavigate(label)} className={cn("flex h-9 w-full items-center gap-3 rounded-md px-3 text-left text-sm transition-colors", activeItem === label ? "bg-white text-primary shadow-sm" : "text-blue-50 hover:bg-white/10")}><Icon className="size-4" /><span>{label}</span></button>)}</div></nav><div className="border-t border-white/15 px-5 py-4 text-xs text-blue-100">Offline desktop system</div></aside>;
}
