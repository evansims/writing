"use client";

import dynamic from "next/dynamic";
import { useState, useEffect } from "react";
import { Loader2 } from "lucide-react";

// Dynamically import the EnhancedTextToSpeech component with no SSR
const EnhancedTextToSpeech = dynamic(
  () => import("@/components/EnhancedTextToSpeech"),
  {
    ssr: false,
    loading: () => (
      <div className="flex items-center space-x-2 rounded-md border p-2">
        <Loader2 size={18} className="animate-spin" />
        <span className="text-muted-foreground text-sm">
          Loading audio player...
        </span>
      </div>
    ),
  },
);

interface ContentTextToSpeechProps {
  content: string;
  title: string;
  slug?: string;
}

export default function ContentTextToSpeech({
  content,
  title,
  slug,
}: ContentTextToSpeechProps) {
  const [derivedSlug, setDerivedSlug] = useState<string>("");
  const [hasRendered, setHasRendered] = useState(false);

  // If slug is not provided, try to derive it from window.location
  useEffect(() => {
    // Prevent multiple derivations
    if (hasRendered) return;

    if (slug) {
      setDerivedSlug(slug);
    } else if (typeof window !== "undefined") {
      // Extract slug from path
      const pathname = window.location.pathname;
      // Remove leading and trailing slashes
      const pathSlug = pathname.replace(/^\/|\/$/g, "");
      setDerivedSlug(pathSlug || "");
    }

    setHasRendered(true);
  }, [slug, hasRendered]);

  // Only render the component when we have a slug
  if (!derivedSlug) {
    return null;
  }

  // The audio player is now positioned at the bottom of the viewport,
  // so we don't need the container wrapper anymore
  return (
    <EnhancedTextToSpeech content={content} title={title} slug={derivedSlug} />
  );
}
