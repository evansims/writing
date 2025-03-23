"use client";

import { useEffect, useState, useCallback } from "react";
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
  const [isExpanded, setIsExpanded] = useState(true);
  const [hasScrolled, setHasScrolled] = useState(false);

  // Handle scroll events
  useEffect(() => {
    const handleScroll = () => {
      // Set hasScrolled to true once the user has scrolled down
      if (window.scrollY > 200 && !hasScrolled) {
        setHasScrolled(true);
        setIsExpanded(false);
      } else if (window.scrollY < 150 && hasScrolled) {
        setHasScrolled(false);
        setIsExpanded(true);
      }
    };

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, [hasScrolled]);

  // Expand TOC on hover or focus
  const expandTOC = useCallback(() => {
    setIsExpanded(true);
  }, []);

  // Collapse TOC when leaving, but only if we've scrolled
  const collapseTOC = useCallback(() => {
    if (hasScrolled) {
      setIsExpanded(false);
    }
  }, [hasScrolled]);

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

  // Handle smooth scrolling
  const handleLinkClick = (
    e: React.MouseEvent<HTMLAnchorElement>,
    id: string,
  ) => {
    e.preventDefault();
    setActiveId(id);

    const element = document.getElementById(id);
    if (element) {
      window.scrollTo({
        top: element.offsetTop - 100, // Offset to account for sticky header
        behavior: "smooth",
      });

      // Update URL hash without scrolling
      history.pushState(null, "", `#${id}`);
    }
  };

  if (headings.length === 0) {
    return null;
  }

  // Calculate progress for each heading
  const activeIndex = Math.max(
    headings.findIndex((heading) => heading.id === activeId),
    0,
  );
  const progress = (activeIndex + 1) / headings.length;

  return (
    <nav
      className={cn(
        "toc-nav",
        hasScrolled && !isExpanded ? "toc-collapsed" : "toc-expanded",
      )}
      aria-label="Table of contents"
      onMouseEnter={expandTOC}
      onMouseLeave={collapseTOC}
      onFocus={expandTOC}
      onBlur={(e) => {
        if (!e.currentTarget.contains(e.relatedTarget as Node)) {
          collapseTOC();
        }
      }}
    >
      <h2 className="toc-title text-muted-foreground mb-4 text-sm font-medium tracking-wide uppercase">
        {isExpanded ? "IN THIS ARTICLE" : "READING PROGRESS"}
      </h2>

      <div className="toc-progress-container">
        <div
          className="toc-progress-bar"
          style={{ height: `${progress * 100}%` }}
          aria-hidden="true"
        />
      </div>

      <ul
        className={cn(
          "toc-list space-y-2 text-sm",
          !isExpanded && "toc-list-collapsed",
        )}
      >
        {headings.map((heading, index) => {
          const isActive = activeId === heading.id;
          const isPassed = activeIndex >= index;

          return (
            <li
              key={heading.id}
              className={cn(
                "toc-item",
                isExpanded ? "toc-item-expanded" : "toc-item-collapsed",
                isActive && "toc-item-active",
                isPassed && "toc-item-passed",
              )}
            >
              <Link
                href={`#${heading.id}`}
                className={cn(
                  "hover:text-foreground relative block py-1 transition-colors",
                  isActive
                    ? "text-foreground font-medium"
                    : "text-muted-foreground",
                )}
                onClick={(e) => handleLinkClick(e, heading.id)}
                tabIndex={0}
              >
                <span className="toc-text">{heading.text}</span>
                <span
                  className={cn(
                    "toc-indicator",
                    isActive && "toc-indicator-active",
                  )}
                  aria-hidden="true"
                ></span>
              </Link>
            </li>
          );
        })}
      </ul>
    </nav>
  );
}
