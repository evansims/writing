import { useState, useEffect, useRef, useCallback } from "react";

interface UseTextToSpeechOptions {
  text: string;
  rate?: number;
  pitch?: number;
  lang?: string;
}

export function useTextToSpeech({
  text,
  rate = 1,
  pitch = 1,
  lang = "en-US",
}: UseTextToSpeechOptions) {
  const [isPlaying, setIsPlaying] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const [isMuted, setIsMuted] = useState(false);
  const [progress, setProgress] = useState(0);
  const [currentSentenceIndex, setCurrentSentenceIndex] = useState(0);
  const [currentRate, setCurrentRate] = useState(rate);
  const [isSupported, setIsSupported] = useState(false);

  const utteranceRef = useRef<SpeechSynthesisUtterance | null>(null);
  const previousVolumeRef = useRef<number>(1);
  const sentencesRef = useRef<string[]>([]);

  // Check if speech synthesis is supported
  useEffect(() => {
    const supported =
      typeof window !== "undefined" && "speechSynthesis" in window;
    setIsSupported(supported);

    // Clean up on unmount
    return () => {
      if (supported && window.speechSynthesis.speaking) {
        window.speechSynthesis.cancel();
      }
    };
  }, []);

  // Prepare the text by splitting it into sentences
  useEffect(() => {
    if (!text || !isSupported) return;

    // Clean the text
    const cleanText = text
      .replace(/#{1,6}\s/g, "") // Remove markdown headers
      .replace(/\*\*/g, "") // Remove bold markdown
      .replace(/\*/g, "") // Remove italic markdown
      .replace(/\[([^\]]+)\]\([^)]+\)/g, "$1") // Convert links to just text
      .replace(/```[^`]*```/g, "") // Remove code blocks
      .replace(/`([^`]+)`/g, "$1") // Remove inline code
      .replace(/\n\n/g, ". ") // Replace double line breaks with periods
      .replace(/\n/g, " "); // Replace line breaks with spaces

    // Split into sentences
    sentencesRef.current = cleanText.split(/(?<=[.!?])\s+/);
  }, [text, isSupported]);

  // Create and configure the utterance
  const setupUtterance = useCallback(() => {
    if (!isSupported) return null;

    const utterance = new SpeechSynthesisUtterance();
    utterance.rate = currentRate;
    utterance.pitch = pitch;
    utterance.lang = lang;

    utterance.onend = () => {
      const nextIndex = currentSentenceIndex + 1;
      if (nextIndex < sentencesRef.current.length) {
        setCurrentSentenceIndex(nextIndex);
      } else {
        // End of text
        setIsPlaying(false);
        setIsPaused(false);
        setCurrentSentenceIndex(0);
        setProgress(0);
      }
    };

    utterance.onpause = () => setIsPaused(true);
    utterance.onresume = () => setIsPaused(false);

    // Track progress within a sentence
    utterance.onboundary = (e) => {
      if (e.charIndex && e.charLength) {
        const sentenceProgress = e.charIndex / e.charLength;
        const overallProgress =
          (currentSentenceIndex + sentenceProgress) /
          sentencesRef.current.length;
        setProgress(Math.min(overallProgress * 100, 100));
      }
    };

    return utterance;
  }, [currentRate, pitch, lang, isSupported, currentSentenceIndex]);

  // Speak the current sentence
  useEffect(() => {
    if (!isPlaying || !isSupported || sentencesRef.current.length === 0) return;

    const utterance = setupUtterance();
    if (!utterance) return;

    utterance.text = sentencesRef.current[currentSentenceIndex];
    utteranceRef.current = utterance;

    window.speechSynthesis.speak(utterance);

    return () => {
      if (window.speechSynthesis.speaking) {
        window.speechSynthesis.cancel();
      }
    };
  }, [isPlaying, currentSentenceIndex, setupUtterance, isSupported]);

  // Play function
  const play = useCallback(() => {
    if (!isSupported) return;

    if (isPaused) {
      window.speechSynthesis.resume();
      setIsPaused(false);
    } else {
      setCurrentSentenceIndex(0);
    }

    setIsPlaying(true);
  }, [isPaused, isSupported]);

  // Pause function
  const pause = useCallback(() => {
    if (!isSupported || !isPlaying) return;

    window.speechSynthesis.pause();
    setIsPaused(true);
    setIsPlaying(false);
  }, [isSupported, isPlaying]);

  // Stop function
  const stop = useCallback(() => {
    if (!isSupported) return;

    window.speechSynthesis.cancel();
    setIsPlaying(false);
    setIsPaused(false);
    setCurrentSentenceIndex(0);
    setProgress(0);
  }, [isSupported]);

  // Toggle mute
  const toggleMute = useCallback(() => {
    if (!isSupported || !utteranceRef.current) return;

    if (isMuted) {
      utteranceRef.current.volume = previousVolumeRef.current;
    } else {
      previousVolumeRef.current = utteranceRef.current.volume;
      utteranceRef.current.volume = 0;
    }

    setIsMuted(!isMuted);
  }, [isMuted, isSupported]);

  // Change speech rate
  const changeRate = useCallback(
    (newRate: number) => {
      setCurrentRate(newRate);

      // If currently playing, we need to restart with new rate
      if (isPlaying && isSupported) {
        const currentIdx = currentSentenceIndex;
        stop();

        setTimeout(() => {
          setCurrentSentenceIndex(currentIdx);
          setIsPlaying(true);
        }, 50);
      }
    },
    [isPlaying, isSupported, currentSentenceIndex, stop],
  );

  return {
    isSupported,
    isPlaying,
    isPaused,
    isMuted,
    progress,
    currentRate,
    play,
    pause,
    stop,
    toggleMute,
    changeRate,
  };
}
