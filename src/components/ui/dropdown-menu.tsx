import * as DropdownMenuPrimitive from "@radix-ui/react-dropdown-menu";
import type { ComponentProps } from "react";
import { cn } from "../../lib/utils";
const DropdownMenu = DropdownMenuPrimitive.Root; const DropdownMenuTrigger = DropdownMenuPrimitive.Trigger;
function DropdownMenuContent({ className, ...props }: ComponentProps<typeof DropdownMenuPrimitive.Content>) { return <DropdownMenuPrimitive.Portal><DropdownMenuPrimitive.Content sideOffset={6} className={cn("z-50 min-w-44 rounded-md border border-border bg-white p-1 shadow-lg", className)} {...props} /></DropdownMenuPrimitive.Portal>; }
function DropdownMenuItem({ className, ...props }: ComponentProps<typeof DropdownMenuPrimitive.Item>) { return <DropdownMenuPrimitive.Item className={cn("flex cursor-default items-center rounded px-2.5 py-2 text-sm outline-none data-[highlighted]:bg-muted", className)} {...props} />; }
export { DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem };
