"use client";

import dynamic from "next/dynamic";
import { useState } from "react";

// Dynamically import the TextToSpeech component with no SSR
const TextToSpeech = dynamic(() => import("@/components/TextToSpeech"), {
  ssr: false,
});

interface ContentTextToSpeechProps {
  content: string;
  title: string;
}

export default function ContentTextToSpeech({
  content,
  title,
}: ContentTextToSpeechProps) {
  return <TextToSpeech content={content} title={title} />;
}
