/**
 * Custom event system for audio player to communicate with other components
 */

// Define event names
export const AUDIO_EVENTS = {
  PLAY_SECTION: "audio:playSection",
  AUDIO_STATE_CHANGE: "audio:stateChange",
};

// Define event types
interface PlaySectionEvent extends CustomEvent {
  detail: {
    sectionId: string;
    onlyIfPlaying?: boolean;
  };
}

interface AudioStateChangeEvent extends CustomEvent {
  detail: {
    isPlaying: boolean;
    currentSection?: string;
    currentChunkIndex?: number;
  };
}

// Dispatches an event to play a specific section based on its heading ID
export function playSection(
  sectionId: string,
  onlyIfPlaying: boolean = false,
): void {
  const event = new CustomEvent(AUDIO_EVENTS.PLAY_SECTION, {
    bubbles: true,
    detail: {
      sectionId,
      onlyIfPlaying,
    },
  });

  window.dispatchEvent(event);
}

// Listen for play section events
export function listenToSectionPlay(
  callback: (sectionId: string, onlyIfPlaying: boolean) => void,
): () => void {
  const handleEvent = (event: Event) => {
    const customEvent = event as PlaySectionEvent;
    callback(
      customEvent.detail.sectionId,
      customEvent.detail.onlyIfPlaying || false,
    );
  };

  window.addEventListener(AUDIO_EVENTS.PLAY_SECTION, handleEvent);

  // Return cleanup function
  return () => {
    window.removeEventListener(AUDIO_EVENTS.PLAY_SECTION, handleEvent);
  };
}

// Broadcast audio state changes (playing/paused)
export function broadcastAudioState(
  isPlaying: boolean,
  currentSection?: string,
  currentChunkIndex?: number,
): void {
  const event = new CustomEvent(AUDIO_EVENTS.AUDIO_STATE_CHANGE, {
    bubbles: true,
    detail: {
      isPlaying,
      currentSection,
      currentChunkIndex,
    },
  });

  window.dispatchEvent(event);
}

// Listen for audio state changes
export function listenToAudioStateChange(
  callback: (state: {
    isPlaying: boolean;
    currentSection?: string;
    currentChunkIndex?: number;
  }) => void,
): () => void {
  const handleEvent = (event: Event) => {
    const customEvent = event as AudioStateChangeEvent;
    callback(customEvent.detail);
  };

  window.addEventListener(AUDIO_EVENTS.AUDIO_STATE_CHANGE, handleEvent);

  // Return cleanup function
  return () => {
    window.removeEventListener(AUDIO_EVENTS.AUDIO_STATE_CHANGE, handleEvent);
  };
}
