import { useState, useEffect, useCallback } from "react";

interface Voice {
  voiceURI: string;
  name: string;
  lang: string;
  localService: boolean;
  default: boolean;
}

export function useVoiceSelection(preferredLang = "en-US") {
  const [voices, setVoices] = useState<Voice[]>([]);
  const [selectedVoice, setSelectedVoice] = useState<Voice | null>(null);
  const [loading, setLoading] = useState(true);

  // Function to get and filter voices
  const getVoices = useCallback(() => {
    if (typeof window === "undefined" || !("speechSynthesis" in window)) {
      setLoading(false);
      return;
    }

    // Function to process available voices
    const processVoices = () => {
      const availableVoices = window.speechSynthesis.getVoices();

      if (availableVoices.length === 0) {
        return;
      }

      // Map to our Voice interface and filter by language if needed
      const mappedVoices = availableVoices
        .map((v) => ({
          voiceURI: v.voiceURI,
          name: v.name,
          lang: v.lang,
          localService: v.localService,
          default: v.default,
        }))
        .filter((v) => v.lang.includes(preferredLang) || !preferredLang);

      setVoices(mappedVoices);

      // Select default voice
      if (!selectedVoice && mappedVoices.length > 0) {
        // Try to find a default voice for the preferred language
        const defaultVoice =
          mappedVoices.find(
            (v) => v.default && v.lang.includes(preferredLang),
          ) ||
          mappedVoices.find((v) => v.lang.includes(preferredLang)) ||
          mappedVoices[0];

        setSelectedVoice(defaultVoice);
      }

      setLoading(false);
    };

    // Get voices
    const availableVoices = window.speechSynthesis.getVoices();

    if (availableVoices.length !== 0) {
      processVoices();
    } else {
      // Wait for voices to be loaded
      window.speechSynthesis.onvoiceschanged = processVoices;
    }
  }, [preferredLang, selectedVoice]);

  // Initialize voices on mount
  useEffect(() => {
    getVoices();

    return () => {
      if (typeof window !== "undefined" && "speechSynthesis" in window) {
        window.speechSynthesis.onvoiceschanged = null;
      }
    };
  }, [getVoices]);

  // Function to select a voice by URI
  const selectVoiceByURI = useCallback(
    (uri: string) => {
      const voice = voices.find((v) => v.voiceURI === uri);
      if (voice) {
        setSelectedVoice(voice);
        return true;
      }
      return false;
    },
    [voices],
  );

  return {
    voices,
    selectedVoice,
    selectVoiceByURI,
    loading,
  };
}
