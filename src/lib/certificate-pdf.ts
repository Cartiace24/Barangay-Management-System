import { jsPDF } from "jspdf";
import QRCode from "qrcode";
import type { CertificateData } from "../types/document";

export async function createCertificatePdf(certificate: CertificateData, logo: string | null, signature: string | null) {
  const pdf = new jsPDF({ orientation: "portrait", unit: "mm", format: "a4" });
  const qr = await QRCode.toDataURL(certificate.verificationToken, { margin: 1, width: 320, errorCorrectionLevel: "M" });
  if (logo) pdf.addImage(logo, "PNG", 18, 15, 22, 22);
  pdf.setFont("helvetica", "bold"); pdf.setFontSize(11); pdf.text("REPUBLIC OF THE PHILIPPINES", 105, 17, { align: "center" }); pdf.text(certificate.municipality.toUpperCase(), 105, 23, { align: "center" }); pdf.setFontSize(15); pdf.text(certificate.barangayName.toUpperCase(), 105, 30, { align: "center" });
  pdf.setFont("helvetica", "normal"); pdf.setFontSize(9); pdf.text(`${certificate.municipality}, ${certificate.province}`, 105, 36, { align: "center" }); if (certificate.address) pdf.text(certificate.address, 105, 41, { align: "center" });
  pdf.setDrawColor(16, 46, 84); pdf.setLineWidth(.7); pdf.line(18, 48, 192, 48); pdf.setFont("helvetica", "bold"); pdf.setFontSize(19); pdf.text(certificate.documentType.toUpperCase(), 105, 68, { align: "center" });
  pdf.setFont("helvetica", "normal"); pdf.setFontSize(11); const body = [`TO WHOM IT MAY CONCERN:`, "", `This is to certify that ${certificate.residentName.toUpperCase()} is a resident of ${certificate.barangayName}, ${certificate.municipality}, ${certificate.province}.`, "", `This certificate is issued upon the request of the above-named resident for the following purpose: ${certificate.purpose}.`, "", `Issued this ${new Intl.DateTimeFormat("en-PH", { dateStyle: "long" }).format(new Date(`${certificate.issuedAt}T00:00:00`))}.`]; pdf.text(body, 25, 88, { maxWidth: 150, lineHeightFactor: 1.7 });
  if (signature) pdf.addImage(signature, "PNG", 132, 180, 42, 18); pdf.setFont("helvetica", "bold"); pdf.setFontSize(11); pdf.text(certificate.authorizedSignatory || "Authorized Signatory", 153, 205, { align: "center" }); pdf.setFont("helvetica", "normal"); pdf.setFontSize(9); pdf.text(certificate.signatoryPosition || "", 153, 211, { align: "center" });
  pdf.addImage(qr, "PNG", 20, 245, 26, 26); pdf.setFontSize(7.5); pdf.text(`Certificate No.: ${certificate.documentNumber}`, 50, 253); pdf.text("Scan QR or verify using the local verification record.", 50, 258); pdf.setTextColor(90); pdf.text(`Verification ID: ${certificate.verificationToken}`, 50, 263); pdf.setTextColor(0); pdf.line(18, 276, 192, 276); pdf.setFontSize(7); pdf.text("This locally generated certificate is valid only when its verification record remains active.", 105, 282, { align: "center" });
  return pdf;
}
