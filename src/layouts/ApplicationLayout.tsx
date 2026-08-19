import { useState, type ReactNode } from "react";
import { ApplicationHeader } from "../components/shared/ApplicationHeader";
import { Sidebar } from "../components/shared/Sidebar";

interface ApplicationLayoutProps {
  children: ReactNode;
}

export function ApplicationLayout({ children }: ApplicationLayoutProps) {
  const [activeItem, setActiveItem] = useState("Dashboard");
  return <div className="flex min-h-screen bg-background"><Sidebar activeItem={activeItem} onNavigate={setActiveItem} /><div className="flex min-w-0 flex-1 flex-col"><ApplicationHeader title={activeItem} /><main className="flex-1 overflow-auto p-7">{children}</main></div></div>;
}
