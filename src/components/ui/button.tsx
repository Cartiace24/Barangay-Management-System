import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import type { ButtonHTMLAttributes } from "react";
import { cn } from "../../lib/utils";

const buttonVariants = cva(
  "inline-flex h-9 items-center justify-center gap-2 rounded-md px-3 text-sm font-semibold transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary disabled:pointer-events-none disabled:opacity-50",
  { variants: { variant: { default: "bg-primary text-primary-foreground hover:bg-[#12355f]", secondary: "border border-border bg-white text-foreground hover:bg-muted", destructive: "bg-destructive text-white hover:bg-[#9f2730]", ghost: "text-muted-foreground hover:bg-muted hover:text-foreground" }, size: { default: "h-9", sm: "h-8 px-2.5 text-xs", lg: "h-10 px-4", icon: "h-9 w-9 px-0" } }, defaultVariants: { variant: "default", size: "default" } },
);
interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement>, VariantProps<typeof buttonVariants> { asChild?: boolean; }
function Button({ className, variant, size, asChild = false, ...props }: ButtonProps) { const Comp = asChild ? Slot : "button"; return <Comp className={cn(buttonVariants({ variant, size }), className)} {...props} />; }
export { Button, buttonVariants };
