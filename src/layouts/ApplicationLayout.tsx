import { useState, type ReactNode } from "react";
import { ApplicationHeader } from "../components/shared/ApplicationHeader";
import { Sidebar } from "../components/shared/Sidebar";
import type { BarangayProfileSummary } from "../types/barangay";
import type { SessionUser } from "../types/auth";

interface ApplicationLayoutProps {
  children: ReactNode;
  profile: BarangayProfileSummary;
  user: SessionUser;
  onLogout: () => void;
  onChangePassword: () => void;
  onActiveChange?: (item: string) => void;
}

export function ApplicationLayout({ children, profile, user, onLogout, onChangePassword, onActiveChange }: ApplicationLayoutProps) {
  const [activeItem, setActiveItem] = useState("Dashboard");
  const navigate = (item: string) => { setActiveItem(item); onActiveChange?.(item); };
  return <div className="flex min-h-screen min-w-[1024px] bg-background"><Sidebar profile={profile} user={user} activeItem={activeItem} onNavigate={navigate} /><div className="flex min-w-0 flex-1 flex-col"><ApplicationHeader title={activeItem} user={user} onLogout={onLogout} onChangePassword={onChangePassword} /><main className="flex-1 overflow-auto p-7">{children}</main></div></div>;
}
