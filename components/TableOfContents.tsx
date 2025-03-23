"use client";

import { useEffect, useState, useCallback, useRef } from "react";
import Link from "next/link";
import { cn } from "@/lib/utils";
import { ArrowUp } from "lucide-react";

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
  const [isSticky, setIsSticky] = useState(false);
  const tocRef = useRef<HTMLElement>(null);
  const activeIdRef = useRef<string>(activeId);
  const activeIdTimeoutRef = useRef<NodeJS.Timeout | null>(null);

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

      // Check if TOC is sticky
      if (tocRef.current) {
        const tocTop = tocRef.current.getBoundingClientRect().top;
        setIsSticky(window.scrollY > 100);
      }
    };

    // Initial check
    handleScroll();

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

  // Scroll to top
  const scrollToTop = useCallback(() => {
    window.scrollTo({
      top: 0,
      behavior: "smooth",
    });
  }, []);

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

    // Keep a reference to the latest active ID
    activeIdRef.current = activeId;

    // Enhanced function to update the active ID with debouncing
    const updateActiveId = (newId: string) => {
      // Don't update if it's the same ID to prevent re-renders
      if (newId === activeIdRef.current) return;

      // Clear any existing timeout
      if (activeIdTimeoutRef.current) {
        clearTimeout(activeIdTimeoutRef.current);
      }

      // Set a timeout to update the active ID
      activeIdTimeoutRef.current = setTimeout(() => {
        setActiveId(newId);
        activeIdRef.current = newId;
      }, 100); // 100ms debounce
    };

    // Track which headings are visible and their visibility ratio
    const visibleHeadings = new Map<string, number>();

    // Set up intersection observer with multiple thresholds for smoother transitions
    const callback = (entries: IntersectionObserverEntry[]) => {
      // Update the visibility map
      entries.forEach((entry) => {
        const id = entry.target.id;

        if (entry.isIntersecting) {
          visibleHeadings.set(id, entry.intersectionRatio);
        } else {
          visibleHeadings.delete(id);
        }
      });

      // If we have visible headings, find the most prominent one
      if (visibleHeadings.size > 0) {
        // Sort by intersection ratio (more visible = higher priority)
        const sortedHeadings = Array.from(visibleHeadings.entries()).sort(
          (a, b) => b[1] - a[1],
        );

        // Update the active ID to the most visible heading
        updateActiveId(sortedHeadings[0][0]);
      } else {
        // If no headings are visible, find the one just above the viewport
        const headingsAbove = entries
          .filter(
            (entry) =>
              !entry.isIntersecting && entry.boundingClientRect.top < 0,
          )
          .sort((a, b) => b.boundingClientRect.top - a.boundingClientRect.top);

        if (headingsAbove.length > 0 && headingsAbove[0].target.id) {
          updateActiveId(headingsAbove[0].target.id);
        }
      }
    };

    const observer = new IntersectionObserver(callback, {
      rootMargin: "-20px 0px -20px 0px", // Small margin to trigger a bit before headers enter/leave viewport
      threshold: [0, 0.25, 0.5, 0.75, 1], // Multiple thresholds for smoother transitions
    });

    headingElements.forEach((heading) => {
      observer.observe(heading);
    });

    return () => {
      observer.disconnect();
      // Clear any pending timeout on cleanup
      if (activeIdTimeoutRef.current) {
        clearTimeout(activeIdTimeoutRef.current);
      }
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

  // Filter to get only H2 headings for the collapsed view
  const h2Headings = headings.filter((heading) => heading.level === 2);

  // Find which H2 section is active
  const activeH2 = headings.find((heading) => heading.id === activeId);
  let activeH2Id = activeH2?.id;

  // If the active heading is not an H2, find the parent H2
  if (activeH2 && activeH2.level > 2) {
    // Find the closest H2 that comes before this heading
    const h2Index = headings.findIndex((h) => h.id === activeH2Id);
    for (let i = h2Index; i >= 0; i--) {
      if (headings[i].level === 2) {
        activeH2Id = headings[i].id;
        break;
      }
    }
  }

  // Calculate which H2 sections have been passed
  const h2Progress = h2Headings.map((h2) => {
    const h2Index = headings.findIndex((h) => h.id === h2.id);
    const isPassed = activeIndex >= h2Index;
    const isActive = h2.id === activeH2Id;
    return {
      ...h2,
      isPassed,
      isActive,
    };
  });

  return (
    <nav
      ref={tocRef}
      className={cn(
        "toc-nav",
        hasScrolled && !isExpanded ? "toc-collapsed" : "toc-expanded",
        isSticky && "is-sticky",
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
      <div className="toc-header">
        {isSticky ? (
          <button
            onClick={scrollToTop}
            className="toc-top-link text-muted-foreground hover:text-foreground mb-4 flex items-center gap-1 text-sm font-medium transition-colors"
            aria-label="Scroll to top"
          >
            <ArrowUp size={14} />
            <span>TOP</span>
          </button>
        ) : (
          <h2 className="toc-title text-muted-foreground mb-4 text-sm font-medium tracking-wide uppercase">
            ON THIS PAGE
          </h2>
        )}
      </div>

      {/* Main TOC list - always exists but transforms between states */}
      <div
        className={cn("toc-content", !isExpanded && "toc-content-collapsed")}
      >
        <ul className="toc-list space-y-2 text-sm">
          {headings.map((heading, index) => {
            const isActive = activeId === heading.id;
            const isPassed =
              activeIndex >= headings.findIndex((h) => h.id === heading.id);
            const isChildHeading = heading.level > 2;

            // Find parent H2 for this heading
            let parentH2Index = -1;
            if (isChildHeading) {
              const headingIndex = headings.findIndex(
                (h) => h.id === heading.id,
              );

              // Look backward to find the most recent H2
              for (let i = headingIndex; i >= 0; i--) {
                if (headings[i].level === 2) {
                  parentH2Index = i;
                  break;
                }
              }
            }

            // Determine heading status for styling
            const headingStatus = isActive
              ? "active"
              : isPassed
                ? "passed"
                : "upcoming";

            return (
              <li
                key={heading.id}
                className={cn(
                  "toc-item",
                  `toc-item-${headingStatus}`,
                  isChildHeading && "toc-item-nested",
                  heading.level === 2 && "toc-item-h2",
                )}
              >
                <Link
                  href={`#${heading.id}`}
                  className={cn(
                    "hover:text-foreground relative block py-1 transition-colors",
                    `toc-link-${headingStatus}`,
                  )}
                  onClick={(e) => handleLinkClick(e, heading.id)}
                  tabIndex={0}
                >
                  <span className="toc-line-indicator"></span>
                  <span className="toc-text">{heading.text}</span>
                </Link>
              </li>
            );
          })}
        </ul>

        {/* Progress indicator for collapsed state - integrated with the list */}
        <div className="toc-progress-indicator">
          {h2Progress.map((h2) => (
            <button
              key={h2.id}
              onClick={(e) => {
                e.preventDefault();
                const element = document.getElementById(h2.id);
                if (element) {
                  window.scrollTo({
                    top: element.offsetTop - 100,
                    behavior: "smooth",
                  });
                  setActiveId(h2.id);
                }
              }}
              className={cn(
                "progress-line",
                h2.isPassed && "progress-line-passed",
                h2.isActive && "progress-line-active",
              )}
              aria-label={`Jump to section: ${h2.text}`}
            />
          ))}
        </div>
      </div>
    </nav>
  );
}
