import { useState, type FormEvent } from "react";
import { ArrowLeft, Save } from "lucide-react";
import { Button } from "../components/ui/button";
import { Card, CardContent, CardHeader } from "../components/ui/card";
import { Input } from "../components/ui/input";
import { Select } from "../components/ui/select";
import type { Resident, ResidentInput, ResidentSex, ResidentStatus } from "../types/resident";

interface ResidentFormProps { resident?: Resident; onCancel: () => void; onSave: (input: ResidentInput) => Promise<void>; }
type FormValues = Record<keyof ResidentInput, string>;

const emptyValues: FormValues = { firstName: "", middleName: "", lastName: "", suffix: "", birthDate: "", sex: "unspecified", civilStatus: "", nationality: "Filipino", contactNumber: "", email: "", address: "", barangay: "", municipality: "", province: "", occupation: "", status: "active" };
function toValues(resident?: Resident): FormValues {
  if (!resident) return emptyValues;
  return { firstName: resident.firstName, middleName: resident.middleName ?? "", lastName: resident.lastName, suffix: resident.suffix ?? "", birthDate: resident.birthDate ?? "", sex: resident.sex, civilStatus: resident.civilStatus ?? "", nationality: resident.nationality, contactNumber: resident.contactNumber ?? "", email: resident.email ?? "", address: resident.address ?? "", barangay: resident.barangay ?? "", municipality: resident.municipality ?? "", province: resident.province ?? "", occupation: resident.occupation ?? "", status: resident.status };
}
function fieldLabel(name: string) { return <label className="mb-1.5 block text-sm font-medium text-foreground" htmlFor={name}>{name}</label>; }

export function ResidentForm({ resident, onCancel, onSave }: ResidentFormProps) {
  const [values, setValues] = useState<FormValues>(() => toValues(resident));
  const [error, setError] = useState<string>();
  const [saving, setSaving] = useState(false);
  const set = (key: keyof FormValues, value: string) => setValues((current) => ({ ...current, [key]: value }));
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (!values.firstName.trim() || !values.lastName.trim()) { setError("First name and last name are required."); return; }
    setSaving(true); setError(undefined);
    try { await onSave({ ...values, sex: values.sex as ResidentSex, status: values.status as ResidentStatus }); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setSaving(false); }
  };
  return <div className="mx-auto max-w-6xl space-y-5">
    <div className="flex items-center justify-between"><div><Button variant="ghost" size="sm" onClick={onCancel}><ArrowLeft className="size-4" />Residents</Button><h1 className="mt-2 text-xl font-semibold">{resident ? "Edit resident" : "New resident"}</h1><p className="text-sm text-muted-foreground">Use the resident’s official information and current barangay address.</p></div>{resident && <p className="text-sm text-muted-foreground">Resident ID: <span className="font-medium text-foreground">{resident.residentCode}</span></p>}</div>
    <form onSubmit={submit} className="space-y-5">
      {error && <div role="alert" className="rounded-md border border-destructive/30 bg-[#fdebed] px-4 py-3 text-sm text-destructive">{error}</div>}
      <Card><CardHeader><div><h2 className="font-semibold">Personal information</h2><p className="text-sm text-muted-foreground">Required fields are marked with an asterisk.</p></div></CardHeader><CardContent className="grid grid-cols-2 gap-x-5 gap-y-4">
        <div>{fieldLabel("First name *")}<Input id="First name *" value={values.firstName} onChange={(e) => set("firstName", e.target.value)} required /></div>
        <div>{fieldLabel("Last name *")}<Input id="Last name *" value={values.lastName} onChange={(e) => set("lastName", e.target.value)} required /></div>
        <div>{fieldLabel("Middle name")}<Input id="Middle name" value={values.middleName} onChange={(e) => set("middleName", e.target.value)} /></div>
        <div>{fieldLabel("Suffix")}<Input id="Suffix" placeholder="e.g., Jr., III" value={values.suffix} onChange={(e) => set("suffix", e.target.value)} /></div>
        <div>{fieldLabel("Date of birth")}<Input id="Date of birth" type="date" value={values.birthDate} onChange={(e) => set("birthDate", e.target.value)} /></div>
        <div>{fieldLabel("Sex")}<Select id="Sex" value={values.sex} onChange={(e) => set("sex", e.target.value)}><option value="unspecified">Not specified</option><option value="male">Male</option><option value="female">Female</option><option value="other">Other</option></Select></div>
        <div>{fieldLabel("Civil status")}<Select id="Civil status" value={values.civilStatus} onChange={(e) => set("civilStatus", e.target.value)}><option value="">Not specified</option><option>Single</option><option>Married</option><option>Widowed</option><option>Separated</option></Select></div>
        <div>{fieldLabel("Nationality")}<Input id="Nationality" value={values.nationality} onChange={(e) => set("nationality", e.target.value)} /></div>
      </CardContent></Card>
      <Card><CardHeader><h2 className="font-semibold">Contact and residence</h2></CardHeader><CardContent className="grid grid-cols-2 gap-x-5 gap-y-4">
        <div>{fieldLabel("Contact number")}<Input id="Contact number" inputMode="tel" value={values.contactNumber} onChange={(e) => set("contactNumber", e.target.value)} /></div>
        <div>{fieldLabel("Email address")}<Input id="Email address" type="email" value={values.email} onChange={(e) => set("email", e.target.value)} /></div>
        <div className="col-span-2">{fieldLabel("Address")}<Input id="Address" placeholder="House no., street, sitio/purok" value={values.address} onChange={(e) => set("address", e.target.value)} /></div>
        <div>{fieldLabel("Barangay")}<Input id="Barangay" value={values.barangay} onChange={(e) => set("barangay", e.target.value)} /></div>
        <div>{fieldLabel("Municipality / City")}<Input id="Municipality / City" value={values.municipality} onChange={(e) => set("municipality", e.target.value)} /></div>
        <div>{fieldLabel("Province")}<Input id="Province" value={values.province} onChange={(e) => set("province", e.target.value)} /></div>
        <div>{fieldLabel("Occupation")}<Input id="Occupation" value={values.occupation} onChange={(e) => set("occupation", e.target.value)} /></div>
      </CardContent></Card>
      <Card><CardHeader><h2 className="font-semibold">Record status</h2></CardHeader><CardContent className="grid max-w-[calc(50%-0.625rem)] grid-cols-1 gap-4"><div>{fieldLabel("Status")}<Select id="Status" value={values.status} onChange={(e) => set("status", e.target.value)}><option value="active">Active</option><option value="inactive">Inactive</option><option value="moved_out">Moved out</option><option value="deceased">Deceased</option></Select></div></CardContent></Card>
      <div className="flex justify-end gap-3"><Button type="button" variant="secondary" onClick={onCancel}>Cancel</Button><Button type="submit" disabled={saving}><Save className="size-4" />{saving ? "Saving…" : "Save resident"}</Button></div>
    </form>
  </div>;
}
