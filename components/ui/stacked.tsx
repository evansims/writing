"use client";

import * as React from "react";

import { cn } from "@/lib/utils";

function Stacked({ className, ...props }: React.ComponentProps<"table">) {
  return <div className={cn("flex w-full", className)} {...props} />;
}

function StackedRow({ className, ...props }: React.ComponentProps<"tr">) {
  return (
    <div
      className={cn(
        "hover:bg-muted/50 data-[state=selected]:bg-muted transition-colors",
        className,
      )}
      {...props}
    />
  );
}

function StackedCell({ className, ...props }: React.ComponentProps<"td">) {
  return (
    <div
      className={cn(
        "p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
        className,
      )}
      {...props}
    />
  );
}

export { Stacked, StackedRow, StackedCell };
