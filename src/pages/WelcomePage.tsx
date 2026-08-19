import { BarangayIdentity } from "../components/shared/BarangayIdentity";
import { developmentBarangayProfile } from "../lib/development-profile";

export function WelcomePage() {
  return (
    <main className="welcome-page">
      <section className="welcome-card" aria-labelledby="welcome-heading">
        <BarangayIdentity profile={developmentBarangayProfile} />
        <div className="welcome-card__content">
          <p className="welcome-card__status">System foundation ready</p>
          <h1 id="welcome-heading">Barangay Management System</h1>
          <p>
            This offline-first desktop application is ready for its core barangay
            management modules.
          </p>
        </div>
        <p className="welcome-card__note">
          The barangay details shown are development placeholders. They will be
          supplied by local application settings in a later phase.
        </p>
      </section>
    </main>
  );
}
