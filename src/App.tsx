import "./App.css";
import { useEffect, useState } from "react";
import { ApplicationLayout } from "./layouts/ApplicationLayout";
import { DesignSystemPage } from "./pages/DesignSystemPage";
import { SetupWizardPage } from "./pages/SetupWizardPage";
import { BarangayProfileSettingsPage } from "./pages/BarangayProfileSettingsPage";
import { UserManagementPage } from "./pages/UserManagementPage";
import { LoginPage } from "./pages/LoginPage";
import { ModulePlaceholderPage } from "./pages/ModulePlaceholderPage";
import { ResidentsPage } from "./pages/ResidentsPage";
import { HouseholdsPage } from "./pages/HouseholdsPage";
import { DocumentsPage } from "./pages/DocumentsPage";
import { BlotterPage } from "./pages/BlotterPage";
import { changeOwnPassword, logout } from "./services/auth-service";
import { getBarangayProfile } from "./services/barangay-profile-service";
import type { BarangayProfileSummary } from "./types/barangay";
import type { SessionUser } from "./types/auth";

function App() {
  const [profile, setProfile] = useState<BarangayProfileSummary | null | undefined>(undefined);
  const [activeItem, setActiveItem] = useState("Dashboard");
  const [authenticated, setAuthenticated] = useState(false);
  const [user, setUser] = useState<SessionUser | null>(null);
  const [selectedResidentId, setSelectedResidentId] = useState<number>();
  useEffect(() => { getBarangayProfile().then(setProfile).catch(() => setProfile(null)); }, []);
  if (profile === undefined) return <main className="grid min-h-screen place-items-center text-sm text-muted-foreground">Loading local configuration…</main>;
  if (profile === null) return <SetupWizardPage onComplete={(configuredProfile) => { setProfile(configuredProfile); setAuthenticated(false); }} />;
  if (!authenticated) return <LoginPage profile={profile} onAuthenticated={(sessionUser) => { setUser(sessionUser); setAuthenticated(true); }} />;
  if (!user) return <LoginPage profile={profile} onAuthenticated={(sessionUser) => { setUser(sessionUser); setAuthenticated(true); }} />;
  const leave = async () => { await logout(); setUser(null); setAuthenticated(false); };
  const changePassword = async () => { const currentPassword=window.prompt("Enter your current password:"); if(!currentPassword)return; const newPassword=window.prompt("Enter a new password (at least 10 characters):"); if(!newPassword)return; try{await changeOwnPassword(currentPassword,newPassword);window.alert("Password changed successfully.");}catch(error){window.alert(error instanceof Error?error.message:String(error));} };
  const openResident = (residentId: number) => { setSelectedResidentId(residentId); setActiveItem("Residents"); };
  const page = activeItem === "Settings" ? <div className="space-y-8"><BarangayProfileSettingsPage profile={profile} onSaved={setProfile} /><UserManagementPage user={user} /></div> : activeItem === "Dashboard" ? <DesignSystemPage /> : activeItem === "Residents" ? <ResidentsPage user={user} initialResidentId={selectedResidentId} onInitialResidentOpened={() => setSelectedResidentId(undefined)} /> : activeItem === "Households" ? <HouseholdsPage user={user} onViewResident={openResident} /> : activeItem === "Documents" ? <DocumentsPage user={user} /> : activeItem === "Blotter" ? <BlotterPage user={user} /> : <ModulePlaceholderPage title={activeItem} />;
  return <ApplicationLayout profile={profile} user={user} onLogout={() => void leave()} onChangePassword={() => void changePassword()} onActiveChange={setActiveItem}>{page}</ApplicationLayout>;
}

export default App;
