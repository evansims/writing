export interface ReadingItem {
  title: string;
  author: string;
  url: string;
}

export interface ContentItem {
  slug: string;
  title: string;
  body: string;
  description?: string;
  created?: string;
  updated?: string;
  tags: string[];
  banner?: string;
  url: string;
  topic?: string;
  type?: string;
  reading?: ReadingItem[];
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
  types: string[] = [],
): Promise<ContentItem[]> {
  const r = await fetch(
    `${apiUrl}/api/content/?limit=${limit}&types=${types.join(",")}`,
    {
      next: { revalidate: 60 },
    },
  );

  if (!r.ok) {
    throw new Error(`Failed to fetch content: ${r.statusText}`);
  }

  const j = await r.json();
  return j.pages;
}
