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
  const sections = useRef<Array<{ id: string; top: number; bottom: number }>>(
    [],
  );
  const isScrollingRef = useRef<boolean>(false);
  const scrollTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const lastScrollY = useRef<number>(0);

  // Listen for audio state changes
  useEffect(() => {
    const cleanup = listenToAudioStateChange((state) => {
      setIsAudioPlaying(state.isPlaying);
    });

    return cleanup;
  }, []);

  // Calculate section positions and set up scroll event
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

    // Calculate section positions
    const calculateSectionPositions = () => {
      const allSections = [];

      // Add the intro/title section - this should go from the top of the page
      // to just before the first heading that's not the H1
      if (h1Element instanceof HTMLElement) {
        // Find the first non-H1 heading to determine the bottom of intro section
        const firstNonH1Index = items.findIndex((item) => item.level > 1);
        const firstNonH1Id =
          firstNonH1Index > -1 ? items[firstNonH1Index].id : null;

        const introBottom = firstNonH1Id
          ? (document.getElementById(firstNonH1Id)?.offsetTop ||
              document.body.scrollHeight) - 10
          : document.body.scrollHeight;

        allSections.push({
          id: h1Element.id || "intro",
          top: 0,
          bottom: introBottom,
        });
      }

      // Add each heading as a section
      items.forEach((item, i) => {
        // Skip the H1 as it's already covered by the intro section
        if (item.level === 1 && h1Element && item.id === h1Element.id) {
          return;
        }

        const element = document.getElementById(item.id);
        if (!element) return;

        const top = element.offsetTop - 10; // Small offset
        const bottom =
          i < items.length - 1
            ? (document.getElementById(items[i + 1].id)?.offsetTop ||
                document.body.scrollHeight) - 10
            : document.body.scrollHeight;

        allSections.push({
          id: item.id,
          top,
          bottom,
        });
      });

      sections.current = allSections;
    };

    // Initial calculation
    calculateSectionPositions();

    // Recalculate on window resize
    window.addEventListener("resize", calculateSectionPositions);

    // Handle scroll with throttling and hysteresis
    const handleScroll = () => {
      // Skip if already processing a scroll event, but don't block for too long
      if (isScrollingRef.current) {
        // Still schedule the next check to ensure we don't get stuck
        if (scrollTimeoutRef.current) {
          clearTimeout(scrollTimeoutRef.current);
        }

        scrollTimeoutRef.current = setTimeout(() => {
          isScrollingRef.current = false;
          // Try again after a short delay
          handleScroll();
        }, 100);
        return;
      }

      isScrollingRef.current = true;

      // Use requestAnimationFrame for smoother handling
      window.requestAnimationFrame(() => {
        const scrollY = window.scrollY;
        const viewportHeight = window.innerHeight;
        const SCROLL_BUFFER = 100; // Hysteresis buffer
        const h1Bottom =
          h1Element instanceof HTMLElement
            ? h1Element.getBoundingClientRect().bottom + window.scrollY
            : 0;

        // Handle TOC collapse/expand
        if (scrollY > h1Bottom && !hasScrolled) {
          setHasScrolled(true);
          setIsExpanded(false);
        } else if (scrollY < h1Bottom - 100 && hasScrolled) {
          setHasScrolled(false);
          setIsExpanded(true);
        }

        // Check if TOC is sticky
        if (tocRef.current) {
          setIsSticky(scrollY > 100);
        }

        // Find which section we're in
        const scrollPosition = scrollY + 150; // Use a fixed position from the top for more reliable detection
        let newActiveId = "";

        // Special case: ensure introduction is active when at top of page
        // This takes priority over all other section detection
        if (scrollY < 150) {
          if (h1Id) {
            // Always set intro as active when at top, regardless of previous state
            setActiveId(h1Id);
            activeIdRef.current = h1Id;
            // Store the current scroll position for next comparison
            lastScrollY.current = scrollY;
            // Reset scrolling flag immediately to be responsive
            isScrollingRef.current = false;
            return;
          } else if (activeIdRef.current !== "intro") {
            // Fallback if h1Id is not set
            setActiveId("intro");
            activeIdRef.current = "intro";
            lastScrollY.current = scrollY;
            isScrollingRef.current = false;
            return;
          }
        }

        // Apply hysteresis to prevent jumping between sections too easily
        const isScrollingDown = scrollY > lastScrollY.current;
        const detectionOffset = isScrollingDown ? 0 : SCROLL_BUFFER;

        // Find the appropriate section
        for (const section of sections.current) {
          // Check if we're within this section's bounds
          if (
            scrollPosition >= section.top - detectionOffset &&
            scrollPosition <
              section.bottom + (isScrollingDown ? SCROLL_BUFFER : 0)
          ) {
            // Get section ID, but check if it's a sub-heading (H3-H6)
            const sectionHeading = headings.find((h) => h.id === section.id);

            if (sectionHeading && sectionHeading.level > 2) {
              // If it's an H3-H6, find the parent H2 instead
              const parentH2Id = findParentH2Id(section.id);
              if (parentH2Id) {
                newActiveId = parentH2Id;
              } else {
                newActiveId = section.id; // Fallback if no parent found
              }
            } else {
              // For H1 and H2, use the section ID directly
              newActiveId = section.id;
            }

            break; // Take the first matching section
          }
        }

        // If we didn't find a section but have scrolled down, use the last section
        if (!newActiveId && sections.current.length > 0 && scrollY > 0) {
          const lastSection = sections.current[sections.current.length - 1];
          if (scrollPosition >= lastSection.top) {
            newActiveId = lastSection.id;
          }
        }

        // Only update if the active section has changed
        if (newActiveId && activeIdRef.current !== newActiveId) {
          setActiveId(newActiveId);
          activeIdRef.current = newActiveId;
        }

        // Store the current scroll position for next comparison
        lastScrollY.current = scrollY;

        // Always reset the scrolling flag when done
        isScrollingRef.current = false;
      });
    };

    // Initial check
    handleScroll();

    // Set up scroll listener with passive option for performance
    window.addEventListener("scroll", handleScroll, { passive: true });

    // Clean up
    return () => {
      window.removeEventListener("scroll", handleScroll);
      window.removeEventListener("resize", calculateSectionPositions);

      if (scrollTimeoutRef.current) {
        clearTimeout(scrollTimeoutRef.current);
      }
    };
  }, [hasScrolled, h1Id]);

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

  // Function to handle the title link click
  const handleTitleClick = (e: React.MouseEvent) => {
    e.preventDefault();

    // Find the main content and its first child
    const mainContent = document.querySelector(".main-content");
    const h1 = mainContent?.querySelector("h1");

    if (h1) {
      // Scroll to the H1 (title)
      window.scrollTo({
        top: 0, // Always go to the very top
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

  // Handle smooth scrolling
  const handleLinkClick = (
    e: React.MouseEvent<HTMLAnchorElement>,
    id: string,
  ) => {
    e.preventDefault();

    // Find proper active ID - if it's a sub-heading, use its parent H2
    const heading = headings.find((h) => h.id === id);
    const activeIdToSet =
      heading && heading.level > 2 ? findParentH2Id(id) || id : id;

    setActiveId(activeIdToSet);

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

  // Is the introduction active?
  const isIntroActive =
    activeId === h1Id ||
    activeId === "intro" ||
    activeId === "" ||
    (sections.current.length > 0 && sections.current[0]?.id === activeId);

  // Calculate which H2 sections have been passed
  const h2Progress = h2Headings.map((h2) => {
    const h2Index = headings.findIndex((h) => h.id === h2.id);

    // Only consider a section passed if:
    // 1. We've scrolled down (as tracked by lastScrollY)
    // 2. The activeIndex is past this heading's index
    // 3. The active section is not the introduction/H1
    const isPassed =
      lastScrollY.current > 100 && !isIntroActive && activeIndex > h2Index;

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
      <div className="toc-header"></div>

      {/* Main TOC list - always exists but transforms between states */}
      <div
        className={cn("toc-content", !isExpanded && "toc-content-collapsed")}
      >
        <ul className="toc-list space-y-2 text-sm">
          {/* Title link - always first */}
          <li
            className={cn(
              "toc-item",
              "toc-item-h1",
              isIntroActive
                ? "toc-item-active"
                : lastScrollY.current > 100
                  ? "toc-item-passed"
                  : "toc-item-upcoming",
            )}
          >
            <Link
              href="#"
              className={cn(
                "hover:text-foreground relative flex items-center gap-2 py-1 transition-colors",
                isIntroActive
                  ? "toc-link-active"
                  : lastScrollY.current > 100
                    ? "toc-link-passed"
                    : "toc-link-upcoming",
              )}
              onClick={handleTitleClick}
              tabIndex={0}
            >
              <span className="toc-line-indicator"></span>
              <span className="toc-text font-semibold">Introduction</span>
            </Link>
          </li>

          {/* Regular headings */}
          {headings.map((heading, index) => {
            const isActive = activeId === heading.id;

            // Use the same improved passed logic we use for h2Progress
            const isPassed =
              lastScrollY.current > 100 &&
              activeIndex > headings.findIndex((h) => h.id === heading.id);

            const isChildHeading = heading.level > 2;

            // Skip the H1 that's already shown as the introduction
            if (heading.level === 1 && heading.id === h1Id) {
              return null;
            }

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
                  <span className="toc-text">
                    {isChildHeading && "— "}
                    {heading.text}
                  </span>
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
                : lastScrollY.current > 100
                  ? "progress-line-passed"
                  : "",
            )}
            aria-label={`Jump to introduction`}
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
                  if (isAudioPlaying) {
                    playSection(h2.id, true);
                  }
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
