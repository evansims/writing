"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { cn } from "@/lib/utils";

interface TOCItem {
  id: string;
  text: string;
  level: number;
}

export default function TableOfContents() {
  const [headings, setHeadings] = useState<TOCItem[]>([]);
  const [activeId, setActiveId] = useState<string>("");

  useEffect(() => {
    const articleContent = document.querySelector(".content");

    if (!articleContent) return;

    const headingElements = articleContent.querySelectorAll(
      "h1, h2, h3, h4, h5, h6",
    );
    const items: TOCItem[] = Array.from(headingElements).map((heading) => {
      const id = heading.id;
      const level = parseInt(heading.tagName.substring(1));
      return {
        id,
        text: heading.textContent || "",
        level,
      };
    });

    setHeadings(items);

    // Set up intersection observer
    const callback = (entries: IntersectionObserverEntry[]) => {
      // Find the first heading that is intersecting and closest to the top
      const intersecting = entries.filter((entry) => entry.isIntersecting);

      if (intersecting.length > 0) {
        // Sort by how close they are to the top of the viewport
        const sorted = [...intersecting].sort((a, b) => {
          const aDistance = Math.abs(a.boundingClientRect.top);
          const bDistance = Math.abs(b.boundingClientRect.top);
          return aDistance - bDistance;
        });

        // Set the active ID to the closest heading
        if (sorted[0].target.id) {
          setActiveId(sorted[0].target.id);
        }
      } else if (entries.length > 0) {
        // If no headings are intersecting, find the one that was most recently in view
        // This is determined by checking if the heading is above the viewport
        const headingsAbove = entries.filter(
          (entry) => entry.boundingClientRect.top < 0,
        );

        if (headingsAbove.length > 0) {
          // Sort to find the one closest to the viewport
          const sorted = [...headingsAbove].sort((a, b) => {
            return b.boundingClientRect.top - a.boundingClientRect.top;
          });

          if (sorted[0].target.id) {
            setActiveId(sorted[0].target.id);
          }
        }
      }
    };

    const observer = new IntersectionObserver(callback, {
      rootMargin: "0px 0px -80% 0px",
      threshold: 0,
    });

    headingElements.forEach((heading) => {
      observer.observe(heading);
    });

    return () => {
      observer.disconnect();
    };
  }, []);

  if (headings.length === 0) {
    return null;
  }

  return (
    <nav className="toc-nav" aria-label="Table of contents">
      <h2 className="text-muted-foreground mb-4 text-sm font-medium tracking-wide uppercase">
        IN THIS ARTICLE
      </h2>
      <ul className="space-y-2 text-sm">
        {headings.map((heading) => (
          <li
            key={heading.id}
            style={{
              paddingLeft: `${(heading.level - 1) * 0.75}rem`,
            }}
          >
            <Link
              href={`#${heading.id}`}
              className={cn(
                "hover:text-foreground block py-1 transition-colors",
                activeId === heading.id
                  ? "text-foreground font-medium"
                  : "text-muted-foreground",
              )}
              onClick={() => setActiveId(heading.id)}
            >
              {heading.text}
            </Link>
          </li>
        ))}
      </ul>
    </nav>
  );
}
