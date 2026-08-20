/**
 * Temporary presentation model for the application identity.
 * Persistent settings will replace this once local storage is introduced.
 */
export interface BarangayProfileSummary {
  id: number;
  name: string;
  municipality: string;
  province: string;
  address: string | null;
  contactNumber: string | null;
  email: string | null;
  logoPath: string | null;
  authorizedSignatory: string | null;
  signatoryPosition: string | null;
  signaturePath: string | null;
}
