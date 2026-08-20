import { Construction } from "lucide-react";
import { EmptyState } from "../components/ui/empty-state";
export function ModulePlaceholderPage({ title }: { title: string }) { return <div className="mx-auto max-w-7xl"><EmptyState icon={Construction} title={`${title} module`} description="This area is ready in the authenticated application shell. Its business workflows will be added in a later step." /></div>; }
