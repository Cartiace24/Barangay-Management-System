import type { HTMLAttributes } from "react";
import { cn } from "../../lib/utils";
function Card({ className, ...props }: HTMLAttributes<HTMLDivElement>) { return <div className={cn("rounded-lg border border-border bg-card", className)} {...props} />; }
function CardHeader({ className, ...props }: HTMLAttributes<HTMLDivElement>) { return <div className={cn("flex items-center justify-between border-b border-border px-5 py-4", className)} {...props} />; }
function CardContent({ className, ...props }: HTMLAttributes<HTMLDivElement>) { return <div className={cn("p-5", className)} {...props} />; }
export { Card, CardHeader, CardContent };
