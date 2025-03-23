"use client";

import { useRouter } from "next/navigation";

interface TopicLinkProps {
  topic: string;
  className?: string;
}

export default function TopicLink({ topic, className = "" }: TopicLinkProps) {
  const router = useRouter();

  const handleClick = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    router.push(`/${topic.toLowerCase()}`);
  };

  return (
    <span
      className={`cursor-pointer hover:underline ${className}`}
      onClick={handleClick}
    >
      {topic}
    </span>
  );
}
