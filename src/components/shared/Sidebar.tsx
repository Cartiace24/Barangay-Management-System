import { Building2, FileText, Home, Landmark, Settings, ShieldAlert, Users, UserRoundCog, WalletCards } from "lucide-react";
import type { NavigationItem } from "../../types/navigation";
import { cn } from "../../lib/utils";
import { developmentBarangayProfile } from "../../lib/development-profile";

const navigationItems: NavigationItem[] = [
  { label: "Dashboard", icon: Home }, { label: "Residents", icon: Users }, { label: "Households", icon: Building2 }, { label: "Documents", icon: FileText }, { label: "Blotter", icon: ShieldAlert }, { label: "Payments", icon: WalletCards }, { label: "Reports", icon: Landmark }, { label: "Officials", icon: UserRoundCog }, { label: "Settings", icon: Settings },
];

interface SidebarProps { activeItem: string; onNavigate: (item: string) => void; }
export function Sidebar({ activeItem, onNavigate }: SidebarProps) {
  return <aside className="flex h-screen w-64 shrink-0 flex-col border-r border-[#102e54] bg-primary text-white"><div className="border-b border-white/15 px-5 py-5"><div className="flex items-center gap-3"><div className="grid size-10 place-items-center rounded-md border border-white/35 bg-white/10"><Landmark className="size-5" /></div><div className="min-w-0"><p className="text-xs font-bold leading-4 tracking-wide">{developmentBarangayProfile.name}</p><p className="mt-0.5 text-xs leading-4 text-blue-100">{developmentBarangayProfile.municipality}, {developmentBarangayProfile.province}</p></div></div></div><nav className="flex-1 px-3 py-4" aria-label="Main navigation"><p className="mb-2 px-2 text-[11px] font-bold tracking-[0.12em] text-blue-200">MAIN MENU</p><div className="space-y-1">{navigationItems.map(({ label, icon: Icon }) => <button key={label} type="button" onClick={() => onNavigate(label)} className={cn("flex h-9 w-full items-center gap-3 rounded-md px-3 text-left text-sm transition-colors", activeItem === label ? "bg-white text-primary shadow-sm" : "text-blue-50 hover:bg-white/10")}><Icon className="size-4" /><span>{label}</span></button>)}</div></nav><div className="border-t border-white/15 px-5 py-4 text-xs text-blue-100">Offline desktop system<br />Configuration required before use</div></aside>;
}
