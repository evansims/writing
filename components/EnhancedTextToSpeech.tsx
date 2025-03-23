"use client";

import { useState, useEffect, useRef } from "react";
import { Play, Pause, SkipForward, SkipBack, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { cn } from "@/lib/utils";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";

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

interface TextToSpeechProps {
  content: string;
  title: string;
  slug: string;
  className?: string;
}

export default function EnhancedTextToSpeech({
  content,
  title,
  slug,
  className,
}: TextToSpeechProps) {
  // State for audio playback
  const [isPlaying, setIsPlaying] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [playbackRate, setPlaybackRate] = useState(1);
  const [progress, setProgress] = useState(0);
  const [highlightedElementId, setHighlightedElementId] = useState<
    string | null
  >(null);

  // State for content chunks
  const [audioChunks, setAudioChunks] = useState<AudioChunk[]>([]);
  const [currentChunkIndex, setCurrentChunkIndex] = useState(0);
  const [pageMetadata, setPageMetadata] = useState<PageMetadata | null>(null);
  const [error, setError] = useState<string | null>(null);

  // References
  const audioRef = useRef<HTMLAudioElement | null>(null);
  // Add a ref to track the current chunk index to avoid closure issues
  const currentIndexRef = useRef<number>(0);

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
    console.log(`Updated currentIndexRef to ${currentChunkIndex}`);
  }, [currentChunkIndex]);

  // Fetch audio metadata on component mount
  useEffect(() => {
    const fetchMetadata = async () => {
      try {
        setIsLoading(true);
        setError(null);

        const apiUrl = getApiUrl();
        console.log(`Fetching audio metadata from: ${apiUrl}`);
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
        console.error("Error fetching audio metadata:", err);
        setError("Failed to load audio information");
      } finally {
        setIsLoading(false);
      }
    };

    if (slug) {
      fetchMetadata();
    }
  }, [slug]); // Explicit dependency on slug to prevent unnecessary reruns

  // Create audio element on mount
  useEffect(() => {
    const audio = new Audio();
    audio.preload = "auto";

    // Auto-progression tracking
    let transitioning = false;

    // Basic event handlers
    const handlePlay = () => {
      console.log("Audio play event");
      setIsPlaying(true);
    };

    const handlePause = () => {
      console.log("Audio pause event");
      setIsPlaying(false);
    };

    // Auto-progression handler for when audio ends
    const handleEnded = () => {
      // Prevent multiple calls
      if (transitioning) {
        console.log("Already transitioning, ignoring ended event");
        return;
      }

      transitioning = true;
      console.log("Audio ended event - preparing for next chunk");

      // Manually get current index from ref
      const currentIdx = currentIndexRef.current;
      const nextIdx = currentIdx + 1;

      if (nextIdx < audioChunks.length) {
        console.log(`Moving from chunk ${currentIdx} to ${nextIdx}`);

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
            console.log(`Playing next chunk ${nextIdx}`);
            playNextChunkDirectly(nextIdx, audio)
              .then(() => {
                console.log("Auto-progression successful");
                // Re-attach event handler
                audio.onended = handleEnded;
                transitioning = false;
              })
              .catch((error) => {
                console.error("Error in auto-progression:", error);
                transitioning = false;
                setError("Error advancing to next section");
              });
          }, 300);
        }, 0);
      } else {
        console.log("Reached the end of all chunks");
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
    audio.onerror = (e) => {
      console.error("Audio error:", e);
      setError("Error playing audio");
      setIsPlaying(false);
      setIsLoading(false);
    };
    audio.onloadstart = () => setIsLoading(true);
    audio.oncanplaythrough = () => {
      console.log("Audio canplaythrough event");
      setIsLoading(false);
    };

    // Set initial properties
    audio.playbackRate = playbackRate;
    audio.volume = 1.0;

    // Store reference
    audioRef.current = audio;

    // Cleanup function
    return () => {
      console.log("Cleaning up audio element");

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

  // Handle highlighting the current chunk during playback
  useEffect(() => {
    if (isPlaying && audioChunks[currentChunkIndex]) {
      const chunkId = audioChunks[currentChunkIndex].id;
      highlightChunk(chunkId);
    }

    return () => {
      // Remove highlight when component unmounts or playback stops
      if (highlightedElementId) {
        removeHighlight();
      }
    };
  }, [isPlaying, currentChunkIndex, audioChunks]);

  // Play a specific chunk
  const playChunk = async (index: number) => {
    console.log(
      `playChunk called with index: ${index} (total chunks: ${audioChunks.length})`,
    );

    if (!audioChunks || index < 0 || index >= audioChunks.length) {
      console.warn(`Invalid chunk index: ${index}, not playing`);
      return;
    }

    try {
      // First update the state
      setCurrentChunkIndex(index);
      // Update the ref immediately for tracking
      currentIndexRef.current = index;
      console.log(`Updated currentIndexRef in playChunk to ${index}`);

      setIsLoading(true);
      setError(null);

      const chunk = audioChunks[index];
      const apiUrl = getApiUrl();

      // Construct the URL for the chunk
      let audioUrl = `${apiUrl}/${chunk.id}`;

      console.log(`Fetching audio from: ${audioUrl}`);

      // Try to pre-fetch the next chunk while this one is playing
      prefetchNextChunk(index);

      // Check if the API is working by making a health check request
      // Use the base URL of the current API endpoint rather than a hardcoded path
      const baseApiUrl = `/api${apiUrl.split("/api")[1].split("/").slice(0, 2).join("/")}`;
      console.log(`Checking API health at: ${baseApiUrl}`);

      try {
        const healthCheck = await fetch(baseApiUrl);
        if (!healthCheck.ok) {
          console.error(
            `API health check failed with status: ${healthCheck.status}`,
          );
          // Continue anyway since this is just a precaution
        } else {
          const healthData = await healthCheck.json();
          if (healthData.status === "WARNING" || !healthData.api_key_valid) {
            console.warn("API health check warning:", healthData);
            setError(
              "Text-to-speech API key is invalid or missing. Please contact the site administrator.",
            );
            setIsLoading(false);
            return;
          }
        }
      } catch (healthError) {
        console.error("Failed to check API health:", healthError);
        // Continue anyway since this is just a precaution
      }

      try {
        console.log(`Fetching audio data for chunk ${index}: ${chunk.id}`);
        const response = await fetch(audioUrl);

        if (!response.ok) {
          // Handle specific error codes
          if (response.status === 500) {
            throw new Error(
              "Server error generating audio. The API key might be invalid or missing.",
            );
          } else if (response.status === 404) {
            console.error(`Audio chunk not found at URL: ${audioUrl}`);
            throw new Error("Audio chunk not found. Try refreshing the page.");
          } else {
            console.error(
              `Failed to fetch audio with status: ${response.status} ${response.statusText}`,
            );
            throw new Error(`Failed to fetch audio: ${response.statusText}`);
          }
        }

        const audioBlob = await response.blob();

        // Check if blob is empty
        if (audioBlob.size === 0) {
          console.error("Received empty audio file from server");
          throw new Error(
            "Received empty audio file. The API key might be invalid or there was a server error.",
          );
        }

        console.log(
          `Successfully fetched audio chunk with size: ${audioBlob.size} bytes`,
        );

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
          console.log(`Setting audio source to blob URL for chunk ${index}`);
          audioRef.current.src = audioObjectUrl;
          audioRef.current.load();

          try {
            console.log(`Starting playback for chunk ${index}`);
            await audioRef.current.play();
            setIsPlaying(true);
          } catch (err) {
            console.error("Failed to play audio:", err);
            setError(
              "Failed to play audio. Your browser may be blocking autoplay.",
            );
          }
        } else {
          console.error("Audio element reference is null");
          throw new Error("Audio player not initialized");
        }
      } catch (fetchError) {
        console.error("Error fetching audio chunk:", fetchError);
        throw fetchError;
      }
    } catch (err) {
      console.error("Error loading audio chunk:", err);
      setError(err instanceof Error ? err.message : "Failed to load audio");
    } finally {
      setIsLoading(false);
    }
  };

  // Pre-fetch next chunk to ensure smooth playback
  const prefetchNextChunk = (currentIndex: number) => {
    const nextIndex = currentIndex + 1;
    if (nextIndex >= audioChunks.length) {
      console.log("No next chunk to prefetch");
      return;
    }

    const nextChunk = audioChunks[nextIndex];
    const apiUrl = getApiUrl();

    // First, try to fetch with generate_all parameter to ensure the audio is generated
    console.log(`Pre-generating audio for next chunk: ${nextChunk.id}`);

    // Make a request to generate the audio file in the background
    fetch(`${apiUrl}?generate_all=true`, {
      method: "GET",
      headers: {
        "Cache-Control": "no-cache",
      },
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error(
            `Failed to trigger audio generation: ${response.status}`,
          );
        }
        return response.json();
      })
      .then((data) => {
        console.log(`Successfully pre-generated audio for next chunks:`, data);
      })
      .catch((err) => {
        console.warn("Error pre-generating audio:", err);
        // Don't need to show this error to the user
      });

    // Also try to pre-fetch the specific next chunk directly
    const nextChunkUrl = `${apiUrl}/${nextChunk.id}`;
    console.log(`Pre-fetching next chunk audio from: ${nextChunkUrl}`);

    // Using a simple HEAD request to get the browser to cache the response
    fetch(nextChunkUrl, {
      method: "HEAD",
      headers: {
        "Cache-Control": "no-cache",
      },
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error(`Failed to pre-fetch next chunk: ${response.status}`);
        }
        console.log(
          `Successfully pre-fetched audio for next chunk: ${nextChunk.id}`,
        );
      })
      .catch((err) => {
        console.warn("Error pre-fetching next chunk:", err);
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
      }
      setIsPlaying(false);
    } else {
      if (audioRef.current && audioRef.current.src) {
        audioRef.current.play();
      } else if (audioChunks.length > 0) {
        playChunk(currentChunkIndex);
      }
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

    // Handle different types of chunks
    if (chunkId === "intro") {
      // For intro section, find the first paragraph
      const firstParagraph = document.querySelector(".main-content > p");
      if (firstParagraph) {
        firstParagraph.classList.add("audio-playing");
        setHighlightedElementId("intro-paragraph");
        scrollToElement(firstParagraph as HTMLElement);
      }
    } else if (chunkId.startsWith("section_")) {
      // For sections, find the corresponding h2 and add the audio-playing class
      const sectionIndex = parseInt(chunkId.replace("section_", ""), 10);
      const h2Elements = document.querySelectorAll(".main-content h2");

      if (sectionIndex < h2Elements.length) {
        const sectionH2 = h2Elements[sectionIndex] as HTMLElement;
        sectionH2.classList.add("audio-playing");
        setHighlightedElementId(sectionH2.id || `section-h2-${sectionIndex}`);
        scrollToElement(sectionH2);
      }
    } else if (chunkId === "full_content") {
      // Handle case with no h2 headings
      const firstParagraph = document.querySelector(".main-content > p");
      if (firstParagraph) {
        firstParagraph.classList.add("audio-playing");
        setHighlightedElementId("full-content-paragraph");
        scrollToElement(firstParagraph as HTMLElement);
      }
    }
  };

  // Remove highlight from current elements
  const removeHighlight = () => {
    // Remove all audio-playing classes
    document.querySelectorAll(".audio-playing").forEach((el) => {
      el.classList.remove("audio-playing");
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

    console.log(
      `Scrolling element ${element.tagName} to top position: ${scrollPosition}`,
    );

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
      console.warn(`Invalid chunk index: ${index}, not playing`);
      return;
    }

    try {
      setIsLoading(true);

      const chunk = audioChunks[index];
      const apiUrl = getApiUrl();
      const audioUrl = `${apiUrl}/${chunk.id}`;

      console.log(`Direct fetch for audio from: ${audioUrl}`);

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
      console.log("Starting auto-playback");
      await audioElement.play();

      // Update the UI
      setIsPlaying(true);

      // Highlight the correct chunk
      highlightChunk(chunk.id);
    } catch (error) {
      console.error("Error in direct playback:", error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  // If we have no chunks or encountered an error loading them
  if (audioChunks.length === 0 && !isLoading && !error) {
    return null;
  }

  return (
    <div className={cn("fixed-audio-player", className)}>
      <div className="relative space-y-2">
        {/* First row: Controls and progress slider combined */}
        <div className="flex items-center space-x-3">
          {/* Playback controls */}
          <div className="flex space-x-2">
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    variant="outline"
                    size="icon"
                    disabled={isLoading || audioChunks.length === 0}
                    onClick={previousChunk}
                  >
                    <SkipBack className="h-4 w-4" />
                  </Button>
                </TooltipTrigger>
                <TooltipContent>Previous</TooltipContent>
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
                  >
                    {isLoading ? (
                      <Loader2 className="h-4 w-4 animate-spin" />
                    ) : isPlaying ? (
                      <Pause className="h-4 w-4" />
                    ) : (
                      <Play className="h-4 w-4" />
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
                  >
                    <SkipForward className="h-4 w-4" />
                  </Button>
                </TooltipTrigger>
                <TooltipContent>Next</TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>

          {/* Progress slider - now taking remaining space */}
          <div className="relative flex-1">
            <Slider
              disabled={isLoading || audioChunks.length === 0}
              value={[progress]}
              max={100}
              step={1}
              className="z-10"
              onValueChange={(value: number[]) => {
                if (audioRef.current) {
                  const newTime = (value[0] / 100) * audioRef.current.duration;
                  audioRef.current.currentTime = newTime;
                  setProgress(value[0]);
                }
              }}
            />
          </div>
        </div>

        {/* Second row: Information about current playback */}
        <div className="text-muted-foreground flex justify-between text-xs">
          {audioChunks.length > 0 ? (
            <>
              <span>
                {currentChunkIndex + 1} of {audioChunks.length}
              </span>
              <span className="max-w-[70%] truncate text-right">
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
          <div className="bg-destructive/10 text-destructive mt-2 rounded-md p-3 text-sm">
            <p>{error}</p>
          </div>
        )}
      </div>
    </div>
  );
}
