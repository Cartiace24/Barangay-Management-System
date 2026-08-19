import type { ReactNode } from "react";
interface FormFieldProps { label: string; htmlFor: string; description?: string; error?: string; children: ReactNode; }
function FormField({ label, htmlFor, description, error, children }: FormFieldProps) { return <div className="grid gap-1.5"><label htmlFor={htmlFor} className="text-sm font-semibold text-foreground">{label}</label>{children}{error ? <p className="text-xs text-destructive">{error}</p> : description ? <p className="text-xs text-muted-foreground">{description}</p> : null}</div>; }
export { FormField };
