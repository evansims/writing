export interface ContentItem {
  slug: string;
  title: string;
  description: string;
  content: string;
  created: string;
  updated: string;
  tags: string[];
  banner?: string;
}

export async function getContent(slug: string): Promise<ContentItem> {
  const res = await fetch(
    `${process.env.NEXT_PUBLIC_API_URL || ""}/api/content/${slug}`,
    {
      next: { revalidate: 60 }, // Revalidate every minute
    }
  );

  if (!res.ok) {
    throw new Error(`Failed to fetch content: ${res.statusText}`);
  }

  return res.json();
}

export async function getLatestContent(
  limit: number = 6
): Promise<ContentItem[]> {
  // In a real implementation, we would fetch from an API endpoint that returns latest content
  // For now, we're mocking this data

  // This is just for demonstration, in the real app this would be an API call
  return [
    {
      slug: "about",
      title: "About",
      description: "Learn more about Evan Sims",
      content: "",
      created: "2024-03-01",
      updated: "2024-03-01",
      tags: ["personal"],
    },
    {
      slug: "mindset/curb-your-enthusiasm",
      title: "Curb Your Enthusiasm",
      description: "Thoughts on maintaining perspective",
      content: "",
      created: "2024-02-15",
      updated: "2024-02-15",
      tags: ["mindset", "philosophy"],
    },
  ].slice(0, limit);
}
