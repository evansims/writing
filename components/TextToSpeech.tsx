"use client";

import { useState } from "react";
import {
  Play,
  Pause,
  Volume2,
  VolumeX,
  Settings,
  ChevronDown,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useTextToSpeech } from "@/hooks/useTextToSpeech";
import { useVoiceSelection } from "@/hooks/useVoiceSelection";

interface TextToSpeechProps {
  content: string;
  title: string;
  className?: string;
}

export default function TextToSpeech({
  content,
  title,
  className,
}: TextToSpeechProps) {
  const [showSettings, setShowSettings] = useState(false);
  const [showVoiceSelector, setShowVoiceSelector] = useState(false);

  // Get available voices
  const { voices, selectedVoice, selectVoiceByURI } = useVoiceSelection();

  // Configure text-to-speech with selected voice
  const {
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
  } = useTextToSpeech({
    text: content,
    rate: 1,
    pitch: 1,
    lang: selectedVoice?.lang || "en-US",
    voiceURI: selectedVoice?.voiceURI,
  });

  if (!isSupported) {
    return null; // Don't show the component if TTS is not supported
  }

  return (
    <div
      className={cn(
        "flex items-center space-x-2 rounded-md border p-2",
        className,
      )}
    >
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              size="icon"
              variant="ghost"
              onClick={isPlaying ? pause : play}
              aria-label={isPlaying ? "Pause narration" : "Play narration"}
            >
              {isPlaying ? <Pause size={18} /> : <Play size={18} />}
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>{isPlaying ? "Pause narration" : "Listen to this article"}</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>

      <div className="flex-1">
        <div className="bg-secondary h-2 w-full rounded-full">
          <div
            className="bg-primary h-full rounded-full transition-all duration-200"
            style={{ width: `${progress}%` }}
          />
        </div>
      </div>

      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              size="icon"
              variant="ghost"
              onClick={toggleMute}
              aria-label={isMuted ? "Unmute" : "Mute"}
            >
              {isMuted ? <VolumeX size={18} /> : <Volume2 size={18} />}
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>{isMuted ? "Unmute" : "Mute"}</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>

      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              size="icon"
              variant="ghost"
              onClick={() => {
                setShowSettings(!showSettings);
                setShowVoiceSelector(false);
              }}
              aria-label="Playback settings"
            >
              <Settings size={18} />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Playback settings</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>

      {/* Voice selector button */}
      {voices.length > 1 && (
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                size="icon"
                variant="ghost"
                onClick={() => {
                  setShowVoiceSelector(!showVoiceSelector);
                  setShowSettings(false);
                }}
                aria-label="Select voice"
              >
                <ChevronDown size={18} />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Select voice</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      )}

      {showSettings && (
        <div className="bg-popover absolute top-12 right-0 z-10 min-w-[200px] rounded-md border p-4 shadow-md">
          <p className="text-muted-foreground mb-2 text-sm font-medium">
            Playback Speed
          </p>
          <div className="flex justify-between space-x-2">
            {[0.5, 0.75, 1, 1.25, 1.5, 2].map((speedRate) => (
              <Button
                key={speedRate}
                variant={currentRate === speedRate ? "default" : "outline"}
                size="sm"
                onClick={() => changeRate(speedRate)}
                className="flex-1"
              >
                {speedRate}x
              </Button>
            ))}
          </div>
        </div>
      )}

      {showVoiceSelector && voices.length > 0 && (
        <div className="bg-popover absolute top-12 right-0 z-10 max-h-[300px] min-w-[240px] overflow-y-auto rounded-md border p-4 shadow-md">
          <p className="text-muted-foreground mb-2 text-sm font-medium">
            Select Voice
          </p>
          <div className="flex flex-col space-y-1">
            {voices.map((voice) => (
              <Button
                key={voice.voiceURI}
                variant={
                  selectedVoice?.voiceURI === voice.voiceURI
                    ? "default"
                    : "outline"
                }
                size="sm"
                onClick={() => {
                  selectVoiceByURI(voice.voiceURI);
                  setShowVoiceSelector(false);

                  // If playing, restart with new voice
                  if (isPlaying) {
                    stop();
                    setTimeout(() => play(), 50);
                  }
                }}
                className="justify-start overflow-hidden text-ellipsis whitespace-nowrap"
              >
                {voice.name} ({voice.lang})
              </Button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
