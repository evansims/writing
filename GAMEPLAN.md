# Text-to-Speech Feature Implementation with Eleven Labs

## Overview

This gameplan outlines the steps to implement a text-to-speech feature using the Eleven Labs API. The implementation will allow visitors to listen to content like an audiobook, with advanced features such as section-by-section playback, text highlighting during playback, and caching of generated audio.

## Backend Implementation

- [ ] **Set up Eleven Labs SDK**

  - [ ] Install the Eleven Labs Python SDK in the backend environment
  - [ ] Configure API keys and environment variables
  - [ ] Test the SDK with a simple example to confirm it works

- [ ] **Create Audio Generation and Caching System**

  - [ ] Define a function to split markdown content into logical chunks (paragraphs/sections)
  - [ ] Implement checksum generation for content chunks to use as filenames
  - [ ] Create a directory structure to store audio files alongside content
  - [ ] Implement a caching mechanism to avoid regenerating unchanged content

- [ ] **Implement the `/api/audio` Endpoint**

  - [ ] Create a new API endpoint at `/api/audio` (audio.py)
  - [ ] Implement slug-based content retrieval similar to `/api/content`
  - [ ] Add parameters to request specific sections or paragraphs
  - [ ] Implement logic to check for existing audio files before generation
  - [ ] Add error handling and appropriate status codes
  - [ ] Add support for streamed responses when multiple chunks are requested

- [ ] **Add Audio Metadata API**
  - [ ] Create an endpoint to return metadata about available audio chunks
  - [ ] Include timestamps, checksums, and section identifiers
  - [ ] Return a mapping between content sections and audio files

## Frontend Implementation

- [ ] **Update the TextToSpeech Component**

  - [ ] Modify the component to fetch audio from the backend API
  - [ ] Implement a loading indicator for audio generation
  - [ ] Add error handling for failed API requests
  - [ ] Implement audio playback using the Web Audio API or HTML5 Audio

- [ ] **Implement Advanced Audio Player Features**

  - [ ] Create queue system for playing multiple audio chunks sequentially
  - [ ] Add controls for play, pause, skip forward/backward
  - [ ] Implement progress tracking across multiple audio files
  - [ ] Add volume control and speed adjustment

- [ ] **Text Highlighting during Playback**

  - [ ] Create a mapping system between audio timestamps and text positions
  - [ ] Implement a mechanism to highlight the current text being read
  - [ ] Add smooth scrolling to keep the highlighted text in view
  - [ ] Ensure highlighting works across different devices and screen sizes

- [ ] **Section Navigation**

  - [ ] Allow users to click on any paragraph to start playback from that point
  - [ ] Implement a mini-navigation system to jump between sections
  - [ ] Add visual indicators for sections with available audio

- [ ] **UI Refinements**
  - [ ] Design an intuitive audio player interface
  - [ ] Add accessibility features (ARIA attributes, keyboard shortcuts)
  - [ ] Implement responsive design for mobile devices
  - [ ] Add visual feedback for loading, playback, and errors

## Testing and Optimization

- [ ] **Performance Testing**

  - [ ] Test audio generation performance with various content lengths
  - [ ] Optimize chunk size for balance between responsiveness and quality
  - [ ] Benchmark API response times

- [ ] **User Experience Testing**

  - [ ] Test the feature on different devices and browsers
  - [ ] Gather feedback on the player interface
  - [ ] Ensure smooth playback transitions between chunks

- [ ] **Error Handling and Edge Cases**
  - [ ] Implement graceful degradation when the API is unavailable
  - [ ] Handle content updates that invalidate cached audio
  - [ ] Test with various languages and special characters

## Integration and Deployment

- [ ] **Documentation**

  - [ ] Document the API endpoints and parameters
  - [ ] Create usage examples for the frontend components
  - [ ] Document the caching mechanism and file structure

- [ ] **Final Integration**

  - [ ] Integrate the feature into the main application
  - [ ] Ensure it works with the existing content system
  - [ ] Add feature flags or gradual rollout if needed

- [ ] **Monitoring and Analytics**
  - [ ] Add logging for audio generation and playback
  - [ ] Implement usage tracking for the feature
  - [ ] Set up alerts for API failures or performance issues

## Maintenance Plan

- [ ] **Regular Validation**

  - [ ] Periodically check for stale audio files
  - [ ] Validate that audio matches current content
  - [ ] Monitor Eleven Labs API changes and versioning

- [ ] **Voice Customization (Future Enhancement)**
  - [ ] Allow selecting different voices
  - [ ] Support custom voice models
  - [ ] Add voice preferences to user settings

## Technical Implementation Details

### Backend API Structure

```python
# audio.py (simplified example)
from elevenlabs.client import ElevenLabs
import hashlib
import os

# Initialize Eleven Labs client
client = ElevenLabs(api_key=os.getenv("ELEVENLABS_API_KEY"))

def generate_audio_for_text(text, voice_id):
    """Generate audio using Eleven Labs API"""
    audio = client.text_to_speech.convert(
        text=text,
        voice_id=voice_id,
        model_id="eleven_multilingual_v2"
    )
    return audio

def get_content_chunks(content):
    """Split content into logical chunks for audio generation"""
    # Implementation will depend on content structure
    # Example: split by paragraphs, headings, or custom markers
    return chunks

def calculate_checksum(text):
    """Generate a checksum for a text chunk to use as filename"""
    return hashlib.md5(text.encode()).hexdigest()

def get_audio_path(content_path, chunk_checksum):
    """Determine the path where audio should be stored"""
    content_dir = os.path.dirname(content_path)
    audio_dir = os.path.join(content_dir, "audio")
    os.makedirs(audio_dir, exist_ok=True)
    return os.path.join(audio_dir, f"{chunk_checksum}.mp3")

def handle_audio_request(slug, chunk_id=None):
    """Main handler for audio API endpoint"""
    # Get content similar to content.py
    content = get_content(slug)

    if chunk_id:
        # Handle single chunk request
        chunk = get_chunk_by_id(content, chunk_id)
        checksum = calculate_checksum(chunk)
        audio_path = get_audio_path(content_path, checksum)

        if os.path.exists(audio_path):
            # Return cached audio
            return read_file(audio_path)
        else:
            # Generate new audio
            audio = generate_audio_for_text(chunk, voice_id)
            save_file(audio_path, audio)
            return audio
    else:
        # Handle full content request
        # Could return metadata or generate all chunks
        chunks = get_content_chunks(content)
        return json.dumps({
            "chunks": [{"id": i, "checksum": calculate_checksum(c)} for i, c in enumerate(chunks)]
        })
```

### Frontend Component Structure

```typescript
// TextToSpeech.tsx (simplified example)
"use client";

import { useState, useEffect, useRef } from "react";
import { Play, Pause, SkipForward, SkipBack, Volume2 } from "lucide-react";
import { Button } from "@/components/ui/button";

interface AudioChunk {
  id: string;
  checksum: string;
  text: string;
}

export default function EnhancedTextToSpeech({ content, slug }) {
  const [audioChunks, setAudioChunks] = useState<AudioChunk[]>([]);
  const [currentChunkIndex, setCurrentChunkIndex] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [progress, setProgress] = useState(0);
  const audioRef = useRef<HTMLAudioElement | null>(null);

  // Fetch audio metadata on component mount
  useEffect(() => {
    async function fetchAudioMetadata() {
      try {
        const response = await fetch(`/api/audio/${slug}`);
        const data = await response.json();
        setAudioChunks(data.chunks);
      } catch (error) {
        console.error("Failed to fetch audio metadata:", error);
      }
    }

    fetchAudioMetadata();
  }, [slug]);

  // Play a specific chunk
  async function playChunk(index: number) {
    if (index < 0 || index >= audioChunks.length) return;

    setCurrentChunkIndex(index);
    setIsLoading(true);

    try {
      // Get audio for specific chunk
      const response = await fetch(`/api/audio/${slug}?chunk_id=${audioChunks[index].id}`);
      if (!response.ok) throw new Error("Failed to fetch audio");

      const audioBlob = await response.blob();
      const audioUrl = URL.createObjectURL(audioBlob);

      if (audioRef.current) {
        audioRef.current.src = audioUrl;
        audioRef.current.play();
        setIsPlaying(true);
      }
    } catch (error) {
      console.error("Failed to play audio chunk:", error);
    } finally {
      setIsLoading(false);
    }
  }

  // Handle audio playback controls
  function togglePlayPause() {
    if (audioRef.current) {
      if (isPlaying) {
        audioRef.current.pause();
      } else {
        audioRef.current.play();
      }
      setIsPlaying(!isPlaying);
    } else if (audioChunks.length > 0) {
      playChunk(currentChunkIndex);
    }
  }

  // Update progress and handle chunk transitions
  function handleTimeUpdate() {
    if (audioRef.current) {
      const { currentTime, duration } = audioRef.current;
      setProgress((currentTime / duration) * 100);
    }
  }

  function handleAudioEnded() {
    // Move to next chunk when current one ends
    if (currentChunkIndex < audioChunks.length - 1) {
      playChunk(currentChunkIndex + 1);
    } else {
      setIsPlaying(false);
      setCurrentChunkIndex(0);
      setProgress(0);
    }
  }

  // Highlight text currently being read
  useEffect(() => {
    if (isPlaying && audioChunks[currentChunkIndex]) {
      const textElement = document.getElementById(`text-${audioChunks[currentChunkIndex].id}`);
      if (textElement) {
        textElement.classList.add('highlighted-text');
        textElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }

      // Clean up previous highlights
      return () => {
        document.querySelectorAll('.highlighted-text').forEach(el => {
          el.classList.remove('highlighted-text');
        });
      };
    }
  }, [isPlaying, currentChunkIndex, audioChunks]);

  return (
    <div className="audio-player">
      <audio
        ref={audioRef}
        onTimeUpdate={handleTimeUpdate}
        onEnded={handleAudioEnded}
        onPlay={() => setIsPlaying(true)}
        onPause={() => setIsPlaying(false)}
      />

      <div className="controls">
        <Button
          onClick={() => playChunk(currentChunkIndex - 1)}
          disabled={currentChunkIndex <= 0 || isLoading}
          aria-label="Previous section"
        >
          <SkipBack size={18} />
        </Button>

        <Button
          onClick={togglePlayPause}
          disabled={isLoading || audioChunks.length === 0}
          aria-label={isPlaying ? "Pause" : "Play"}
        >
          {isLoading ? "Loading..." : isPlaying ? <Pause size={18} /> : <Play size={18} />}
        </Button>

        <Button
          onClick={() => playChunk(currentChunkIndex + 1)}
          disabled={currentChunkIndex >= audioChunks.length - 1 || isLoading}
          aria-label="Next section"
        >
          <SkipForward size={18} />
        </Button>

        <div className="progress-bar">
          <div
            className="progress-fill"
            style={{ width: `${progress}%` }}
          />
        </div>
      </div>
    </div>
  );
}
```

## Initial Setup Instructions

1. Install the Eleven Labs SDK:

   ```
   pip install elevenlabs python-dotenv
   ```

2. Set up environment variables:

   ```
   ELEVENLABS_API_KEY=your_api_key_here
   ```

3. Create the directory structure for audio storage:

   ```
   /content
     /topic
       /article
         /audio
           checksum1.mp3
           checksum2.mp3
   ```

4. Implement the backend API first, then integrate with the frontend components
