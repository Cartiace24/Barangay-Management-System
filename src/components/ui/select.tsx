import type { SelectHTMLAttributes } from "react";
import { ChevronDown } from "lucide-react";
import { cn } from "../../lib/utils";
interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> { containerClassName?: string; }
function Select({ className, containerClassName, children, ...props }: SelectProps) { return <div className={cn("relative", containerClassName)}><select className={cn("h-9 w-full appearance-none rounded-md border border-border bg-white px-3 pr-9 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/15 disabled:bg-muted", className)} {...props}>{children}</select><ChevronDown className="pointer-events-none absolute right-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" /></div>; }
export { Select };
