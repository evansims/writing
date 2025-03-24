"use client";

import { useEffect } from "react";

/**
 * Error component for llms-full.txt route
 * Implements proper error handling as per Next.js best practices
 */
export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    // Log the error to an error reporting service
    console.error("LLMs full content error:", error);
  }, [error]);

  // Return plain text for this specialized route
  return new Response(
    "Error: Unable to load full LLMs content. Please try again later.",
    {
      status: 500,
      headers: {
        "Content-Type": "text/plain",
      },
    },
  );
}
