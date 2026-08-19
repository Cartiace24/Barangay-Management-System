import type { HTMLAttributes, TableHTMLAttributes } from "react";
import { cn } from "../../lib/utils";
function Table({ className, ...props }: TableHTMLAttributes<HTMLTableElement>) { return <div className="overflow-x-auto"><table className={cn("w-full caption-bottom text-sm", className)} {...props} /></div>; }
function TableHeader({ className, ...props }: HTMLAttributes<HTMLTableSectionElement>) { return <thead className={cn("border-b border-border bg-[#f8fafc] text-left text-xs font-semibold uppercase tracking-wide text-muted-foreground", className)} {...props} />; }
function TableBody({ className, ...props }: HTMLAttributes<HTMLTableSectionElement>) { return <tbody className={cn("divide-y divide-border", className)} {...props} />; }
function TableRow({ className, ...props }: HTMLAttributes<HTMLTableRowElement>) { return <tr className={cn("hover:bg-[#fafbfd]", className)} {...props} />; }
function TableHead({ className, ...props }: HTMLAttributes<HTMLTableCellElement>) { return <th className={cn("h-10 px-4 font-semibold", className)} {...props} />; }
function TableCell({ className, ...props }: HTMLAttributes<HTMLTableCellElement>) { return <td className={cn("p-4 align-middle", className)} {...props} />; }
export { Table, TableHeader, TableBody, TableRow, TableHead, TableCell };
