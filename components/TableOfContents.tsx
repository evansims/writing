"use client";

import { useEffect, useState, useCallback, useRef } from "react";
import Link from "next/link";
import { cn } from "@/lib/utils";
import { ArrowUp } from "lucide-react";
import { playSection, listenToAudioStateChange } from "@/lib/audioEvents";

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
  const [hasHeadings, setHasHeadings] = useState(false);
  const [isAudioPlaying, setIsAudioPlaying] = useState(false);
  const [h1Id, setH1Id] = useState<string>("");
  const [pageTitle, setPageTitle] = useState<string>("");
  const tocRef = useRef<HTMLElement>(null);
  const activeIdRef = useRef<string>(activeId);
  const activeIdTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Listen for audio state changes
  useEffect(() => {
    const cleanup = listenToAudioStateChange((state) => {
      setIsAudioPlaying(state.isPlaying);
    });

    return cleanup;
  }, []);

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

  // Main effect for setting up heading detection
  useEffect(() => {
    // Only run on client-side
    if (typeof window === "undefined") return;

    const articleContent = document.querySelector(".content");
    if (!articleContent) return;

    // Get the h1 title element
    const h1Element = document.querySelector(".main-content h1");

    // Extract page title and ID
    if (h1Element instanceof HTMLElement) {
      if (h1Element.id) {
        setH1Id(h1Element.id);
      }

      // Get and trim the title text
      const titleText = h1Element.textContent || "";
      const trimmedTitle = titleText.trim();

      if (trimmedTitle) {
        console.log("Found page title:", trimmedTitle);
        setPageTitle(trimmedTitle);
      } else {
        console.warn("Found h1 element but no text content");
      }
    } else {
      console.warn("Could not find main h1 element for title");
    }

    // Collect all headings
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
    setHasHeadings(items.length > 0);

    // Add or remove a class on the document element based on whether headings exist
    if (items.length === 0) {
      document.documentElement.classList.add("no-toc-headings");
    } else {
      document.documentElement.classList.remove("no-toc-headings");
    }

    // Function to determine which heading is active based on scroll position
    const determineActiveHeading = () => {
      const scrollTop = window.scrollY;
      const ACTIVATION_OFFSET = 150; // Distance from top to consider heading "active"

      // Check if we're at the top of the page
      if (scrollTop < 100 && h1Element?.id) {
        setActiveId(h1Element.id);
        return;
      }

      // Find the heading just above the activation line
      let activeHeadingId = "";
      let smallestDistance = Infinity;

      // Iterate through all headings and find the one closest to the top
      Array.from(headingElements).forEach((heading) => {
        if (heading instanceof HTMLElement && heading.id) {
          const headingTop = heading.offsetTop;
          const positionFromTop = headingTop - scrollTop - ACTIVATION_OFFSET;

          // If heading is above activation line but closest to it
          if (positionFromTop <= 0 && positionFromTop > -smallestDistance) {
            activeHeadingId = heading.id;
            smallestDistance = Math.abs(positionFromTop);
          }
        }
      });

      // Update active heading if found
      if (activeHeadingId) {
        setActiveId(activeHeadingId);
      }
    };

    // Throttle scroll handling for performance
    let isScrolling = false;
    const handleScroll = () => {
      if (!isScrolling) {
        window.requestAnimationFrame(() => {
          determineActiveHeading();
          isScrolling = false;
        });
        isScrolling = true;
      }
    };

    // Do an initial check after a short delay to ensure DOM is ready
    setTimeout(determineActiveHeading, 100);

    // Set up scroll listener
    window.addEventListener("scroll", handleScroll);

    return () => {
      window.removeEventListener("scroll", handleScroll);

      // Clear any pending timeout on cleanup
      if (activeIdTimeoutRef.current) {
        clearTimeout(activeIdTimeoutRef.current);
      }
    };
  }, []);

  // Find the parent H2 for a given heading ID
  const findParentH2Id = (headingId: string): string | null => {
    const headingIndex = headings.findIndex((h) => h.id === headingId);
    if (headingIndex === -1) return null;

    const heading = headings[headingIndex];

    // If it's already an H2 or H1, return it directly
    if (heading.level <= 2) return heading.id;

    // Look backwards to find the closest H2
    for (let i = headingIndex; i >= 0; i--) {
      if (headings[i].level === 2) {
        return headings[i].id;
      }
    }

    return null;
  };

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

      // Only trigger audio if already playing, and determine the right section
      if (isAudioPlaying) {
        const heading = headings.find((h) => h.id === id);

        if (heading && heading.level > 2) {
          // For subsections, find the parent H2
          const parentH2Id = findParentH2Id(id);
          if (parentH2Id) {
            // Play the parent section
            playSection(parentH2Id, true);
          }
        } else {
          // Play directly for H1 and H2
          playSection(id, true);
        }
      }
    }
  };

  // Function to handle the title link click
  const handleTitleClick = (e: React.MouseEvent) => {
    e.preventDefault();

    // Find the main content and its first child
    const mainContent = document.querySelector(".main-content");
    const h1 = mainContent?.querySelector("h1");

    if (h1) {
      // Scroll to the H1 (title)
      window.scrollTo({
        top: h1.offsetTop - 100,
        behavior: "smooth",
      });

      // Update active ID if possible
      if (h1.id) {
        setActiveId(h1.id);
        // Update URL hash without scrolling
        history.pushState(null, "", `#${h1.id}`);

        // Only trigger audio playback for intro if already playing
        if (isAudioPlaying) {
          playSection("intro", true);
        }
      }
    }
  };

  if (headings.length === 0) {
    return <div className="toc-container" data-has-headings="false"></div>;
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

  // Is the introduction active?
  const isIntroActive = activeId === h1Id || activeId === "";

  return (
    <nav
      ref={tocRef}
      className={cn(
        "toc-nav",
        hasScrolled && !isExpanded ? "toc-collapsed" : "toc-expanded",
        isSticky && "is-sticky",
      )}
      data-has-headings="true"
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
            YOU ARE HERE
          </h2>
        )}
      </div>

      {/* Main TOC list - always exists but transforms between states */}
      <div
        className={cn("toc-content", !isExpanded && "toc-content-collapsed")}
      >
        <ul className="toc-list space-y-2 text-sm">
          {/* Title link - always first */}
          <li
            className={cn(
              "toc-item",
              "toc-item-h2",
              isIntroActive ? "toc-item-active" : "toc-item-upcoming",
            )}
          >
            <Link
              href="#"
              className={cn(
                "hover:text-foreground relative flex items-center gap-2 py-1 transition-colors",
                isIntroActive ? "toc-link-active" : "toc-link-upcoming",
              )}
              onClick={handleTitleClick}
              tabIndex={0}
            >
              <span className="toc-line-indicator"></span>
              <span className="toc-text font-semibold">
                {pageTitle || "Article"}
              </span>
            </Link>
          </li>

          {/* Regular headings */}
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
                    "hover:text-foreground relative flex items-center gap-2 py-1 transition-colors",
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
          {/* Add a title item to the progress indicator */}
          <button
            onClick={handleTitleClick}
            className={cn(
              "progress-line",
              isIntroActive
                ? "progress-line-active"
                : activeIndex === 0
                  ? "progress-line-passed"
                  : "",
            )}
            aria-label={`Jump to ${pageTitle}`}
          />

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

                  // Trigger audio playback for this section only if already playing
                  playSection(h2.id, true);
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
