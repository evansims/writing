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
        setIsSticky(tocTop <= 10);
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

  // For expanded state, we want to show only relevant sections
  // Start with all H2 headings
  const visibleHeadings = headings.filter((heading) => {
    // Always show H2 headings
    if (heading.level === 2) return true;

    // For H3-H6, only show if they're under the active H2
    if (heading.level > 2) {
      // Find parent H2
      const headingIndex = headings.findIndex((h) => h.id === heading.id);
      let parentH2Index = -1;

      // Look backward to find the most recent H2
      for (let i = headingIndex; i >= 0; i--) {
        if (headings[i].level === 2) {
          parentH2Index = i;
          break;
        }
      }

      // Only show if this heading is under the active H2
      return parentH2Index >= 0 && headings[parentH2Index].id === activeH2Id;
    }

    return false;
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
        {isExpanded ? (
          isSticky ? (
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
              IN THIS ARTICLE
            </h2>
          )
        ) : (
          <button
            onClick={scrollToTop}
            className="toc-top-link-collapsed text-muted-foreground hover:text-foreground mb-4 flex items-center gap-1 text-sm font-medium transition-colors"
            aria-label="Scroll to top"
          >
            <ArrowUp size={14} />
            <span>TOP</span>
          </button>
        )}
      </div>

      {/* Main TOC list - visible when expanded */}
      {isExpanded ? (
        <ul className="toc-list space-y-2 text-sm">
          {headings.map((heading, index) => {
            const isActive = activeId === heading.id;
            const isPassed =
              activeIndex >= headings.findIndex((h) => h.id === heading.id);
            const isChildHeading = heading.level > 2;

            // Find parent H2 for this heading
            let isUnderActiveH2 = false;
            if (isChildHeading) {
              const headingIndex = headings.findIndex(
                (h) => h.id === heading.id,
              );
              let parentH2Index = -1;

              // Look backward to find the most recent H2
              for (let i = headingIndex; i >= 0; i--) {
                if (headings[i].level === 2) {
                  parentH2Index = i;
                  break;
                }
              }

              // Check if under active H2
              isUnderActiveH2 =
                parentH2Index >= 0 && headings[parentH2Index].id === activeH2Id;
            }

            // Only show H2s and child headings under the active H2
            if (isChildHeading && !isUnderActiveH2) {
              return null;
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
                  <span className="toc-text">{heading.text}</span>
                </Link>
              </li>
            );
          })}
        </ul>
      ) : (
        // Progress indicator - visible when collapsed
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
      )}
    </nav>
  );
}
