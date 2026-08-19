import { Info } from "lucide-react";
import type { HTMLAttributes, ReactNode } from "react";
import { cn } from "../../lib/utils";

interface AlertProps extends HTMLAttributes<HTMLDivElement> { title: string; children: ReactNode; }
function Alert({ title, children, className, ...props }: AlertProps) { return <div role="alert" className={cn("flex gap-3 rounded-md border border-[#cbdcf3] bg-[#f3f7fc] p-3 text-sm text-[#244b7c]", className)} {...props}><Info className="mt-0.5 size-4 shrink-0" /><div><p className="font-semibold">{title}</p><div className="mt-0.5 text-[#45648c]">{children}</div></div></div>; }
export { Alert };
