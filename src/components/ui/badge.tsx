import { cva, type VariantProps } from "class-variance-authority";
import type { HTMLAttributes } from "react";
import { cn } from "../../lib/utils";
const badgeVariants = cva("inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold", { variants: { variant: { success: "bg-[#e8f6ec] text-success", warning: "bg-[#fff4df] text-warning", destructive: "bg-[#fdebed] text-destructive", info: "bg-[#eaf2fc] text-info", neutral: "bg-muted text-muted-foreground" } }, defaultVariants: { variant: "neutral" } });
interface BadgeProps extends HTMLAttributes<HTMLSpanElement>, VariantProps<typeof badgeVariants> {}
function Badge({ className, variant, ...props }: BadgeProps) { return <span className={cn(badgeVariants({ variant }), className)} {...props} />; }
export { Badge };
