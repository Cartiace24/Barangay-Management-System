import type { BarangayProfileSummary } from "../../types/barangay";

interface BarangayIdentityProps {
  profile: BarangayProfileSummary;
}

export function BarangayIdentity({ profile }: BarangayIdentityProps) {
  return (
    <div className="barangay-identity">
      <span className="barangay-identity__eyebrow">Republic of the Philippines</span>
      <strong className="barangay-identity__name">{profile.name}</strong>
      <span className="barangay-identity__location">
        {profile.municipality}, {profile.province}
      </span>
    </div>
  );
}
