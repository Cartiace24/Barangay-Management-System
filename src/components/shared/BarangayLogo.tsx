import { Landmark } from "lucide-react";
import { useEffect, useState } from "react";
import { getBrandingImage } from "../../services/barangay-profile-service";

export function BarangayLogo({ size = "size-10" }: { size?: string }) {
  const [source, setSource] = useState<string | null>(null);
  useEffect(() => { getBrandingImage("logo").then(setSource).catch(() => setSource(null)); }, []);
  return source ? <img src={source} alt="Barangay logo or seal" className={`${size} object-contain`} /> : <Landmark className="size-5" />;
}
