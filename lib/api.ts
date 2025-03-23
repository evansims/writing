export interface ContentItem {
  slug: string;
  title: string;
  description?: string;
  created?: string;
  updated?: string;
  tags: string[];
  banner?: string;
  body: string;
  url: string;
  topic?: string;
}

const apiUrl = process.env.NEXT_PUBLIC_API_URL || "http://localhost:5328";

export async function getContent(
  slug: string | string[],
): Promise<ContentItem> {
  let s = Array.isArray(slug) ? slug.join("/") : String(slug);
  s = s.replace(/,/g, "/");

  const r = await fetch(`${apiUrl}/api/content/${s}`, {
    next: { revalidate: 60 },
  });

  if (!r.ok) {
    throw new Error(`Failed to fetch content: ${r.statusText} for ${slug}`);
  }

  const j = await r.json();
  return j.page;
}

export async function getLatestContent(
  limit: number = 6,
): Promise<ContentItem[]> {
  const r = await fetch(`${apiUrl}/api/content/`, {
    next: { revalidate: 60 },
  });

  if (!r.ok) {
    throw new Error(`Failed to fetch content: ${r.statusText}`);
  }

  const j = await r.json();
  return j.pages;
}
