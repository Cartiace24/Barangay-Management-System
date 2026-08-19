import type { LucideIcon } from "lucide-react";
import type { ReactNode } from "react";
interface EmptyStateProps { icon: LucideIcon; title: string; description: string; action?: ReactNode; }
function EmptyState({ icon: Icon, title, description, action }: EmptyStateProps) { return <div className="flex min-h-72 flex-col items-center justify-center rounded-lg border border-dashed border-border bg-white px-6 text-center"><div className="mb-3 rounded-full bg-muted p-3 text-primary"><Icon className="size-6" /></div><h2 className="text-base font-semibold">{title}</h2><p className="mt-1 max-w-md text-sm text-muted-foreground">{description}</p>{action ? <div className="mt-5">{action}</div> : null}</div>; }
export { EmptyState };
