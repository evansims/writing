"use client";

import { useState, useEffect, useRef } from "react";
import {
  Play,
  Pause,
  SkipForward,
  SkipBack,
  Loader2,
  Headphones,
  X,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { cn } from "@/lib/utils";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { listenToSectionPlay, broadcastAudioState } from "@/lib/audioEvents";

interface AudioChunk {
  id: string;
  text: string;
  title?: string;
  checksum: string;
  has_audio: boolean;
}

interface PageMetadata {
  slug: string;
  title: string;
}

/**
 * Props for the EnhancedTextToSpeech component
 */
interface TextToSpeechProps {
  /** The text content to be converted to speech */
  content: string;

  /** The title of the content */
  title: string;

  /** The slug identifier for the content, used for API calls */
  slug: string;

  /** Additional CSS classes for the main player element */
  className?: string;

  /** CSS classes for the inner container of the player */
  playerContainerClassName?: string;

  /** CSS classes for the trigger button itself */
  triggerClassName?: string;

  /** CSS classes for the div wrapping the trigger button (for positioning) */
  triggerWrapperClassName?: string;

  /** Custom text for the trigger button (default is "Listen") */
  triggerLabel?: string;

  /** Custom text for when audio is playing (default is "Pause") */
  triggerPauseLabel?: string;
}

/**
 * Enhanced Text-to-Speech component that provides an audio player for content.
 *
 * Features:
 * - Floating audio player that appears when triggered
 * - Customizable trigger button with headphones icon
 * - Section navigation for audio content
 * - Keyboard shortcuts (Esc to close)
 * - Fully accessible with ARIA attributes
 * - Customizable styling through various className props
 */
export default function EnhancedTextToSpeech({
  content,
  title,
  slug,
  className,
  playerContainerClassName,
  triggerClassName,
  triggerWrapperClassName,
  triggerLabel = "Listen",
  triggerPauseLabel = "Pause",
}: TextToSpeechProps) {
  // State for audio playback
  const [isPlaying, setIsPlaying] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [playbackRate, setPlaybackRate] = useState(1);
  const [progress, setProgress] = useState(0);
  const [highlightedElementId, setHighlightedElementId] = useState<
    string | null
  >(null);

  // State for player visibility - hidden by default
  const [isVisible, setIsVisible] = useState(false);

  // State for content chunks
  const [audioChunks, setAudioChunks] = useState<AudioChunk[]>([]);
  const [currentChunkIndex, setCurrentChunkIndex] = useState(0);
  const [pageMetadata, setPageMetadata] = useState<PageMetadata | null>(null);
  const [error, setError] = useState<string | null>(null);

  // References
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const currentIndexRef = useRef<number>(0);
  const playerRef = useRef<HTMLDivElement>(null);
  const sentinelRef = useRef<HTMLDivElement>(null);
  const placeholderRef = useRef<HTMLDivElement>(null);

  // Create a ref for the component container and placeholder
  const containerRef = useRef<HTMLDivElement>(null);
  const observerRef = useRef<HTMLDivElement>(null);
  const [audioPlayerHeight, setAudioPlayerHeight] = useState(0);

  // Add refs to track animation state
  const animationTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const [animationClass, setAnimationClass] = useState("");

  // Determine the API URL based on the slug structure
  const getApiUrl = () => {
    // Remove any leading or trailing slashes to normalize the slug
    const normalizedSlug = slug.replace(/^\/|\/$/g, "");

    // Check if this is a nested path (has folder/page structure)
    if (normalizedSlug.includes("/")) {
      const parts = normalizedSlug.split("/");
      const folder = parts.slice(0, -1).join("/");
      const page = parts[parts.length - 1];

      // Use the specific nested endpoint for better path resolution
      return `/api/audio/${folder}/${page}`;
    }

    // For non-nested paths
    return `/api/audio/${normalizedSlug}`;
  };

  // Update the ref when currentChunkIndex changes
  useEffect(() => {
    currentIndexRef.current = currentChunkIndex;
  }, [currentChunkIndex]);

  // Fetch audio metadata on component mount
  useEffect(() => {
    const fetchMetadata = async () => {
      try {
        setIsLoading(true);
        setError(null);

        const apiUrl = getApiUrl();
        const response = await fetch(apiUrl);

        if (!response.ok) {
          throw new Error(
            `Failed to fetch audio metadata: ${response.statusText}`,
          );
        }

        const data = await response.json();
        setAudioChunks(data.chunks);
        setPageMetadata(data.page);
      } catch (err) {
        setError("Failed to load audio information");
      } finally {
        setIsLoading(false);
      }
    };

    if (slug) {
      fetchMetadata();
    }
  }, [slug]);

  // Create audio element on mount
  useEffect(() => {
    const audio = new Audio();
    audio.preload = "auto";

    // Auto-progression tracking
    let transitioning = false;

    // Basic event handlers
    const handlePlay = () => {
      setIsPlaying(true);
    };

    const handlePause = () => {
      setIsPlaying(false);
      removeHighlight();
    };

    // Auto-progression handler for when audio ends
    const handleEnded = () => {
      // Prevent multiple calls
      if (transitioning) {
        return;
      }

      transitioning = true;

      // Manually get current index from ref
      const currentIdx = currentIndexRef.current;
      const nextIdx = currentIdx + 1;

      if (nextIdx < audioChunks.length) {
        // Important: Remove event handler during transition to prevent double firing
        audio.onended = null;

        // Set up a timeout to advance to the next chunk
        setTimeout(() => {
          // Update the state
          setCurrentChunkIndex(nextIdx);
          currentIndexRef.current = nextIdx;

          // Wait for React to process state updates
          setTimeout(() => {
            // Play the next chunk
            playNextChunkDirectly(nextIdx, audio)
              .then(() => {
                // Re-attach event handler
                audio.onended = handleEnded;
                transitioning = false;
              })
              .catch((error) => {
                transitioning = false;
                setError("Error advancing to next section");
              });
          }, 300);
        }, 0);
      } else {
        setIsPlaying(false);
        setCurrentChunkIndex(0);
        currentIndexRef.current = 0;
        setProgress(0);
        removeHighlight();
        transitioning = false;
      }
    };

    // Set up event handlers
    audio.onplay = handlePlay;
    audio.onpause = handlePause;
    audio.onended = handleEnded;
    audio.ontimeupdate = handleTimeUpdate;
    audio.onerror = () => {
      setError("Error playing audio");
      setIsPlaying(false);
      setIsLoading(false);
    };
    audio.onloadstart = () => setIsLoading(true);
    audio.oncanplaythrough = () => {
      setIsLoading(false);
    };

    // Set initial properties
    audio.playbackRate = playbackRate;
    audio.volume = 1.0;

    // Store reference
    audioRef.current = audio;

    // Cleanup function
    return () => {
      audio.pause();
      audio.onplay = null;
      audio.onpause = null;
      audio.onended = null;
      audio.ontimeupdate = null;
      audio.onerror = null;
      audio.onloadstart = null;
      audio.oncanplaythrough = null;

      if (audio.src && audio.src.startsWith("blob:")) {
        URL.revokeObjectURL(audio.src);
      }

      audio.src = "";
      audio.remove();

      removeHighlight();
    };
  }, [audioChunks.length]);

  // Update audio properties when they change
  useEffect(() => {
    if (audioRef.current) {
      audioRef.current.playbackRate = playbackRate;
    }
  }, [playbackRate]);

  // Effect to ensure highlight state matches playback state
  useEffect(() => {
    if (!isPlaying) {
      // When not playing, ensure all highlights are removed
      removeHighlight();
    } else if (isPlaying && audioChunks[currentChunkIndex]) {
      // When playing, ensure the correct section is highlighted
      const chunkId = audioChunks[currentChunkIndex].id;
      highlightChunk(chunkId);
    }
  }, [isPlaying, currentChunkIndex, audioChunks]);

  // Play a specific chunk
  const playChunk = async (index: number) => {
    if (!audioChunks || index < 0 || index >= audioChunks.length) {
      return;
    }

    try {
      // First update the state
      setCurrentChunkIndex(index);
      // Update the ref immediately for tracking
      currentIndexRef.current = index;

      setIsLoading(true);
      setError(null);

      const chunk = audioChunks[index];
      const apiUrl = getApiUrl();

      // Construct the URL for the chunk
      let audioUrl = `${apiUrl}/${chunk.id}`;

      // Start prefetching next chunks in the background
      // We'll do this for the next 2 chunks to ensure smooth playback
      for (let i = 1; i <= 2; i++) {
        const nextIndex = index + i;
        if (nextIndex < audioChunks.length) {
          prefetchNextChunk(nextIndex);
        }
      }

      // Check if the API is working by making a health check request
      const baseApiUrl = `/api${apiUrl.split("/api")[1].split("/").slice(0, 2).join("/")}`;

      try {
        const healthCheck = await fetch(baseApiUrl);
        if (!healthCheck.ok) {
          // Continue anyway since this is just a precaution
        } else {
          const healthData = await healthCheck.json();
          if (healthData.status === "WARNING" || !healthData.api_key_valid) {
            setError(
              "Text-to-speech API key is invalid or missing. Please contact the site administrator.",
            );
            setIsLoading(false);
            return;
          }
        }
      } catch (healthError) {
        // Continue anyway since this is just a precaution
      }

      try {
        const response = await fetch(audioUrl);

        if (!response.ok) {
          // Handle specific error codes
          if (response.status === 500) {
            throw new Error(
              "Server error generating audio. The API key might be invalid or missing.",
            );
          } else if (response.status === 404) {
            throw new Error("Audio chunk not found. Try refreshing the page.");
          } else {
            throw new Error(`Failed to fetch audio: ${response.statusText}`);
          }
        }

        const audioBlob = await response.blob();

        // Check if blob is empty
        if (audioBlob.size === 0) {
          throw new Error(
            "Received empty audio file. The API key might be invalid or there was a server error.",
          );
        }

        // Clean up any previous object URL before creating a new one
        if (
          audioRef.current &&
          audioRef.current.src &&
          audioRef.current.src.startsWith("blob:")
        ) {
          URL.revokeObjectURL(audioRef.current.src);
        }

        const audioObjectUrl = URL.createObjectURL(audioBlob);

        if (audioRef.current) {
          audioRef.current.src = audioObjectUrl;
          audioRef.current.load();

          try {
            await audioRef.current.play();
            setIsPlaying(true);
          } catch (err) {
            setError(
              "Failed to play audio. Your browser may be blocking autoplay.",
            );
          }
        } else {
          throw new Error("Audio player not initialized");
        }
      } catch (fetchError) {
        throw fetchError;
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load audio");
    } finally {
      setIsLoading(false);
    }
  };

  // Pre-fetch next chunk to ensure smooth playback
  const prefetchNextChunk = (currentIndex: number) => {
    const nextIndex = currentIndex + 1;
    if (nextIndex >= audioChunks.length) {
      return;
    }

    const nextChunk = audioChunks[nextIndex];
    const apiUrl = getApiUrl();

    // First, try to fetch with generate_all parameter to ensure the audio is generated
    fetch(`${apiUrl}?generate_all=true`, {
      method: "GET",
      headers: {
        "Cache-Control": "no-cache",
      },
    }).catch(() => {
      // Don't need to show this error to the user
    });

    // Also try to pre-fetch the specific next chunk directly
    const nextChunkUrl = `${apiUrl}/${nextChunk.id}`;

    // Using a simple HEAD request to get the browser to cache the response
    fetch(nextChunkUrl, {
      method: "HEAD",
      headers: {
        "Cache-Control": "no-cache",
      },
    }).catch(() => {
      // Don't need to show this error to the user
    });
  };

  // Update progress based on current audio time
  const handleTimeUpdate = () => {
    if (audioRef.current) {
      const currentTime = audioRef.current.currentTime;
      const duration = audioRef.current.duration || 1;
      const calculatedProgress = (currentTime / duration) * 100;
      setProgress(calculatedProgress);
    }
  };

  // Toggle play/pause
  const togglePlayPause = () => {
    if (isPlaying) {
      if (audioRef.current) {
        audioRef.current.pause();
        removeHighlight();
      }
      setIsPlaying(false);
    } else {
      if (audioRef.current && audioRef.current.src) {
        audioRef.current.play();
      } else if (audioChunks.length > 0) {
        playChunk(currentChunkIndex);
      }
      setIsPlaying(true);
    }
  };

  // Toggle play/pause from trigger button (affects visibility too)
  const togglePlayPauseFromTrigger = () => {
    if (isPlaying) {
      // If playing, pause and hide the player
      if (audioRef.current) {
        audioRef.current.pause();
        removeHighlight();
      }
      setIsPlaying(false);
      setIsVisible(false);
    } else {
      // If not playing, show player and start playing
      if (audioRef.current && audioRef.current.src) {
        audioRef.current.play();
      } else if (audioChunks.length > 0) {
        playChunk(currentChunkIndex);
      }
      setIsPlaying(true);
      setIsVisible(true);
    }
  };

  // Skip to next chunk
  const nextChunk = () => {
    const nextIndex = currentChunkIndex + 1;
    if (nextIndex < audioChunks.length) {
      playChunk(nextIndex);
    }
  };

  // Skip to previous chunk
  const previousChunk = () => {
    // If we're more than 3 seconds into a chunk, restart it instead of going to previous
    if (audioRef.current && audioRef.current.currentTime > 3) {
      audioRef.current.currentTime = 0;
    } else {
      const prevIndex = currentChunkIndex - 1;
      if (prevIndex >= 0) {
        playChunk(prevIndex);
      }
    }
  };

  // Highlight the current chunk being played
  const highlightChunk = (chunkId: string) => {
    // First remove any existing highlight
    removeHighlight();

    const currentChunk = audioChunks.find((chunk) => chunk.id === chunkId);
    if (!currentChunk) return;

    // Always add the audio-playing class to the main content article
    const mainContent = document.querySelector("article.main-content");
    if (mainContent) {
      mainContent.classList.add("audio-playing");
    }

    // Handle different types of chunks
    if (chunkId === "intro") {
      // For intro section, find the container div that wraps the title and intro content
      const introContainer = document.querySelector(".intro-section");
      if (introContainer) {
        introContainer.classList.add("audio-playing");
        setHighlightedElementId(".intro-section");

        // Still highlight the title specifically for visual indication
        const h1 = document.querySelector(".main-content h1");
        if (h1) {
          scrollToElement(h1 as HTMLElement);
        }
      }
    } else if (chunkId.startsWith("section_")) {
      // For sections, find the section container div
      const sectionIndex = parseInt(chunkId.replace("section_", ""), 10);
      const h2Elements = document.querySelectorAll(".main-content h2");

      if (sectionIndex < h2Elements.length) {
        const sectionH2 = h2Elements[sectionIndex] as HTMLElement;

        // Find the container div that wraps this section
        // This assumes each h2 and its content are wrapped in a section div
        let sectionContainer = sectionH2.closest("div.content-section");

        // If there's no explicit section container, we'll apply to the h2 and its siblings
        if (sectionContainer) {
          // Apply class to the entire container
          sectionContainer.classList.add("audio-playing");
        }

        setHighlightedElementId(`section-${sectionIndex}`);
        scrollToElement(sectionH2);
      }
    } else if (chunkId === "full_content") {
      // Handle case with no h2 headings - highlight all main content
      const contentContainer = document.querySelector(".main-content");
      if (contentContainer) {
        contentContainer.classList.add("audio-playing");
        setHighlightedElementId("full-content");

        // Scroll to the first visible content element
        const firstContentElement = document.querySelector(
          ".main-content > p, .main-content > ul, .main-content > ol, .main-content > blockquote, .main-content > pre, .main-content > figure",
        );
        if (firstContentElement) {
          scrollToElement(firstContentElement as HTMLElement);
        }
      }
    }
  };

  // Remove highlight from current elements
  const removeHighlight = () => {
    // Remove all audio-playing classes from any elements
    document
      .querySelectorAll(".audio-playing, .audio-playing-title")
      .forEach((el) => {
        el.classList.remove("audio-playing", "audio-playing-title");
      });

    setHighlightedElementId(null);
  };

  // Helper function to scroll to an element with smooth scrolling
  const scrollToElement = (element: HTMLElement) => {
    // Calculate the offset to account for the fixed audio player and any other fixed elements
    const audioPlayerHeight = 120; // Height of the audio player
    const topOffset = 20; // Additional offset for spacing

    // Get the element's position
    const elementRect = element.getBoundingClientRect();
    const absoluteElementTop = window.pageYOffset + elementRect.top;

    // Calculate position to scroll to (accounting for the audio player at the bottom)
    const scrollPosition = absoluteElementTop - topOffset;

    // Perform the scroll with smooth behavior
    window.scrollTo({
      top: scrollPosition,
      behavior: "smooth",
    });
  };

  // Special function for direct progression without state updates
  // This helps avoid race conditions during auto-progression
  const playNextChunkDirectly = async (
    index: number,
    audioElement: HTMLAudioElement,
  ) => {
    if (!audioChunks || index < 0 || index >= audioChunks.length) {
      return;
    }

    try {
      setIsLoading(true);

      const chunk = audioChunks[index];
      const apiUrl = getApiUrl();
      const audioUrl = `${apiUrl}/${chunk.id}`;

      // Try to pre-fetch the next chunk too
      prefetchNextChunk(index);

      // Fetch the audio data
      const response = await fetch(audioUrl);

      if (!response.ok) {
        throw new Error(
          `Failed to fetch audio: ${response.status} ${response.statusText}`,
        );
      }

      const audioBlob = await response.blob();

      if (audioBlob.size === 0) {
        throw new Error("Received empty audio file");
      }

      // Clean up previous object URL
      if (audioElement.src && audioElement.src.startsWith("blob:")) {
        URL.revokeObjectURL(audioElement.src);
      }

      // Create and set new object URL
      const audioObjectUrl = URL.createObjectURL(audioBlob);
      audioElement.src = audioObjectUrl;
      audioElement.load();

      // Play the audio
      await audioElement.play();

      // Update the UI
      setIsPlaying(true);

      // Highlight the correct chunk
      highlightChunk(chunk.id);
    } catch (error) {
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  // Listen for section play events
  useEffect(() => {
    // Handler function to play audio for the clicked section
    const handleSectionPlay = (sectionId: string, onlyIfPlaying: boolean) => {
      // Only process if we have audio chunks
      if (!audioChunks.length) return;

      // If onlyIfPlaying is true, only proceed if audio is already playing
      if (onlyIfPlaying && !isPlaying) {
        return;
      }

      // Find the heading element
      const headingElement = document.getElementById(sectionId);
      if (!headingElement) return;

      // Determine which audio chunk to play based on heading
      let chunkToPlay = -1;

      if (headingElement.tagName === "H1") {
        // If it's the title (H1), play the introduction
        chunkToPlay = audioChunks.findIndex((chunk) => chunk.id === "intro");
      } else if (headingElement.tagName === "H2") {
        // If it's an H2, find its index among other H2s
        const h2Elements = Array.from(
          document.querySelectorAll(".main-content h2"),
        );
        const h2Index = h2Elements.findIndex((h2) => h2.id === sectionId);

        if (h2Index >= 0) {
          chunkToPlay = audioChunks.findIndex(
            (chunk) => chunk.id === `section_${h2Index}`,
          );
        }
      }

      if (chunkToPlay >= 0) {
        playChunk(chunkToPlay);
      }
    };

    // Set up event listener
    const cleanup = listenToSectionPlay(handleSectionPlay);

    // Cleanup function
    return cleanup;
  }, [audioChunks, isPlaying]); // Include isPlaying in dependencies

  // Broadcast audio state changes
  useEffect(() => {
    // Get the current section ID based on the current chunk
    let currentSectionId: string | undefined;

    if (audioChunks[currentChunkIndex]) {
      const chunk = audioChunks[currentChunkIndex];

      if (chunk.id === "intro") {
        // For intro, try to find the H1 title
        const h1 = document.querySelector(".main-content h1");
        if (h1) {
          currentSectionId = h1.id;
        }
      } else if (chunk.id.startsWith("section_")) {
        // For sections, find the corresponding H2
        const sectionIndex = parseInt(chunk.id.replace("section_", ""), 10);
        const h2Elements = document.querySelectorAll(".main-content h2");

        if (sectionIndex < h2Elements.length) {
          currentSectionId = h2Elements[sectionIndex].id;
        }
      }
    }

    // Broadcast current state
    broadcastAudioState(isPlaying, currentSectionId, currentChunkIndex);
  }, [isPlaying, currentChunkIndex, audioChunks]);

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Close the player when Escape is pressed and the player is visible
      if (e.key === "Escape" && isVisible) {
        setIsVisible(false);
      }
    };

    // Add the event listener
    window.addEventListener("keydown", handleKeyDown);

    // Clean up
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [isVisible]);

  // If we have no chunks or encountered an error loading them
  if (audioChunks.length === 0 && !isLoading && !error) {
    return null;
  }

  return (
    <>
      {/* Trigger button wrapper for better positioning control */}
      <div className={cn("mb-8", triggerWrapperClassName)}>
        {/* Trigger button - headphones icon with text that toggles play/pause and visibility */}
        <button
          onClick={togglePlayPauseFromTrigger}
          className={cn(
            "border-muted inline-flex items-center justify-center gap-2 rounded-md border px-3 py-2 text-sm transition-colors",
            isPlaying
              ? "text-primary bg-primary/5 hover:bg-primary/10"
              : "text-muted-foreground hover:bg-accent hover:text-accent-foreground",
            triggerClassName,
          )}
          aria-label={
            isPlaying
              ? `Pause audio narration (${triggerPauseLabel})`
              : `Listen to audio narration (${triggerLabel})`
          }
          title={
            isPlaying
              ? `Pause audio narration (${triggerPauseLabel})`
              : `Listen to audio narration (${triggerLabel})`
          }
        >
          {isPlaying ? (
            <Pause className="h-4 w-4" aria-hidden="true" />
          ) : (
            <Headphones className="h-4 w-4" aria-hidden="true" />
          )}
          <span>{isPlaying ? triggerPauseLabel : triggerLabel}</span>
        </button>
      </div>

      {/* Floating player - always fixed, but conditionally visible */}
      <div
        ref={playerRef}
        className={cn(
          "audio-player bg-background fixed right-0 bottom-0 left-0 z-50 border-t p-4 shadow-lg transition-all duration-300 ease-out",
          isVisible
            ? "translate-y-0 opacity-100"
            : "translate-y-full opacity-0",
          className,
        )}
        aria-hidden={!isVisible}
        role="region"
        aria-label="Audio player"
      >
        <div
          className={cn(
            "layout-content relative container mx-auto space-y-2",
            playerContainerClassName,
          )}
        >
          {/* Close button in top right corner */}
          <Button
            onClick={() => setIsVisible(false)}
            variant="outline"
            size="icon"
            className="absolute top-0 right-3"
            aria-label="Close audio player"
          >
            <X className="h-4 w-4" aria-hidden="true" />
          </Button>

          {/* First row: Controls and progress slider combined */}
          <div className="flex items-center space-x-3">
            {/* Playback controls */}
            <div
              className="flex items-center space-x-2"
              role="group"
              aria-label="Audio playback controls"
            >
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <Button
                      variant="outline"
                      size="icon"
                      disabled={isLoading || audioChunks.length === 0}
                      onClick={previousChunk}
                      aria-label="Previous section"
                      className="flex items-center justify-center"
                    >
                      <SkipBack className="h-4 w-4" aria-hidden="true" />
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent>Previous section</TooltipContent>
                </Tooltip>
              </TooltipProvider>

              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <Button
                      variant="outline"
                      size="icon"
                      disabled={isLoading || audioChunks.length === 0}
                      onClick={togglePlayPause}
                      aria-label={
                        isLoading ? "Loading" : isPlaying ? "Pause" : "Play"
                      }
                      className="flex items-center justify-center"
                    >
                      {isLoading ? (
                        <Loader2
                          className="h-4 w-4 animate-spin"
                          aria-hidden="true"
                        />
                      ) : isPlaying ? (
                        <Pause className="h-4 w-4" aria-hidden="true" />
                      ) : (
                        <Play className="h-4 w-4" aria-hidden="true" />
                      )}
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent>
                    {isLoading ? "Loading" : isPlaying ? "Pause" : "Play"}
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>

              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <Button
                      variant="outline"
                      size="icon"
                      disabled={isLoading || audioChunks.length === 0}
                      onClick={nextChunk}
                      aria-label="Next section"
                      className="flex items-center justify-center"
                    >
                      <SkipForward className="h-4 w-4" aria-hidden="true" />
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent>Next section</TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>

            {/* Progress slider - taking remaining space */}
            <div className="relative flex flex-1 items-center pr-8">
              <Slider
                disabled={isLoading || audioChunks.length === 0}
                value={[progress]}
                max={100}
                step={1}
                className="z-10"
                onValueChange={(value: number[]) => {
                  if (audioRef.current) {
                    const newTime =
                      (value[0] / 100) * audioRef.current.duration;
                    audioRef.current.currentTime = newTime;
                    setProgress(value[0]);
                  }
                }}
                aria-label="Audio progress"
                aria-valuemin={0}
                aria-valuemax={100}
                aria-valuenow={progress}
                aria-valuetext={`${Math.round(progress)}% complete`}
              />
            </div>
          </div>

          {/* Second row: Information about current playback */}
          <div
            className="text-muted-foreground flex items-center justify-between text-xs"
            aria-live="polite"
          >
            {audioChunks.length > 0 ? (
              <>
                <span className="flex-shrink-0">
                  Section {currentChunkIndex + 1} of {audioChunks.length}
                </span>

                {/* Keyboard shortcut information - now in the middle */}
                <span className="mx-2 flex-shrink-0 text-center text-xs">
                  Press{" "}
                  <kbd className="rounded border px-1 py-0.5 text-xs">Esc</kbd>{" "}
                  to close
                </span>

                <span className="max-w-[40%] flex-shrink-0 truncate text-right">
                  {audioChunks[currentChunkIndex]?.title
                    ? audioChunks[currentChunkIndex].title
                    : audioChunks[currentChunkIndex]?.id === "intro"
                      ? "Introduction"
                      : audioChunks[currentChunkIndex]?.text?.slice(0, 40) +
                        "..."}
                </span>
              </>
            ) : (
              <span>No audio available</span>
            )}
          </div>

          {/* Error display */}
          {error && (
            <div
              className="bg-destructive/10 text-destructive mt-2 rounded-md p-3 text-sm"
              role="alert"
            >
              <p>{error}</p>
            </div>
          )}
        </div>
      </div>
    </>
  );
}
