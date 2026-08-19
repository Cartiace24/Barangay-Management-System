import type { InputHTMLAttributes } from "react";
import { cn } from "../../lib/utils";
function Input({ className, ...props }: InputHTMLAttributes<HTMLInputElement>) { return <input className={cn("h-9 w-full rounded-md border border-border bg-white px-3 text-sm outline-none placeholder:text-muted-foreground focus:border-primary focus:ring-2 focus:ring-primary/15 disabled:bg-muted disabled:text-muted-foreground", className)} {...props} />; }
export { Input };
