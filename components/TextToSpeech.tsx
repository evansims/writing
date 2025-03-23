"use client";

import { useState } from "react";
import { Play, Pause, Volume2, VolumeX, Settings } from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useTextToSpeech } from "@/hooks/useTextToSpeech";

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

  // Configure text-to-speech with Will's voice
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
    lang: "en-US",
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
              onClick={() => setShowSettings(!showSettings)}
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
    </div>
  );
}
