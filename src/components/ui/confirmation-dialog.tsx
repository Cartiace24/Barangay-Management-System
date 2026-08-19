import type { ReactNode } from "react";
import { Button } from "./button";
import { Dialog, DialogContent } from "./dialog";
interface ConfirmationDialogProps { open: boolean; onOpenChange: (open: boolean) => void; title: string; description: string; confirmLabel?: string; onConfirm: () => void; trigger?: ReactNode; }
function ConfirmationDialog({ open, onOpenChange, title, description, confirmLabel = "Confirm", onConfirm, trigger }: ConfirmationDialogProps) { return <Dialog open={open} onOpenChange={onOpenChange}>{trigger}<DialogContent><h2 className="text-lg font-semibold">{title}</h2><p className="mt-2 text-sm text-muted-foreground">{description}</p><div className="mt-6 flex justify-end gap-2"><Button variant="secondary" onClick={() => onOpenChange(false)}>Cancel</Button><Button variant="destructive" onClick={onConfirm}>{confirmLabel}</Button></div></DialogContent></Dialog>; }
export { ConfirmationDialog };
