import * as CheckboxPrimitive from "@radix-ui/react-checkbox";
import { Check } from "lucide-react";
import type { ComponentProps } from "react";
import { cn } from "../../lib/utils";
function Checkbox({ className, ...props }: ComponentProps<typeof CheckboxPrimitive.Root>) { return <CheckboxPrimitive.Root className={cn("flex size-4 items-center justify-center rounded border border-border bg-white text-white outline-none data-[state=checked]:border-primary data-[state=checked]:bg-primary", className)} {...props}><CheckboxPrimitive.Indicator><Check className="size-3" /></CheckboxPrimitive.Indicator></CheckboxPrimitive.Root>; }
export { Checkbox };
