"use client";

import Link from "next/link";

interface MainNavigationProps {
  label?: string;
}

export default function MainNavigation({
  label = "Site Navigation",
}: MainNavigationProps) {
  return (
    <nav aria-label={label} className="mt-8 mb-10">
      <ul className="text-muted-foreground flex space-x-6">
        <li>
          <Link href="/">Home</Link>
        </li>
        <li>
          <Link href="/talks">Talks</Link>
        </li>
        <li>
          <Link href="/audio">Podcast</Link>
        </li>
        <li>
          <Link href="/store">Store</Link>
        </li>
        <li>
          <Link href="/about">About</Link>
        </li>
      </ul>
    </nav>
  );
}
