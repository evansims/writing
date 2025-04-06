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

export const getPublicUrl = (): string => {
  if (process.env.VERCEL_URL) {
    return `https://${process.env.VERCEL_URL}`;
  }

  if (process.env.NEXT_PUBLIC_API_URL) {
    return process.env.NEXT_PUBLIC_API_URL;
  }

  return "http://localhost:3000";
};

export async function getContent(
  slug: string | string[],
): Promise<ContentItem> {
  let s = Array.isArray(slug) ? slug.join("/") : String(slug);
  s = s.replace(/,/g, "/");

  console.log(`${getPublicUrl()}/api/content?path=${s}`);

  const r = await fetch(`${getPublicUrl()}/api/content?path=${s}`, {
    next: { revalidate: 60 },
  });

  if (!r.ok) {
    throw new Error(`Failed to fetch content: ${r.statusText} for ${slug}`);
  }

  const j = await r.json();
  return j.pages[0];
}

export async function getLatestContent(
  limit: number = 6,
  types: string[] = [],
): Promise<ContentItem[]> {
  const r = await fetch(
    `${getPublicUrl()}/api/content?limit=${limit}&types=${types.join(",")}`,
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

export async function getLLMs(): Promise<string> {
  const r = await fetch(`${getPublicUrl()}/api/llms`, {
    next: { revalidate: 60 },
  });

  if (!r.ok) {
    throw new Error(`Failed to fetch content: ${r.statusText}`);
  }

  return await r.text();
}

export async function getLLMsFull(): Promise<string> {
  const r = await fetch(`${getPublicUrl()}/api/llms_full`, {
    next: { revalidate: 60 },
  });

  if (!r.ok) {
    throw new Error(`Failed to fetch content: ${r.statusText}`);
  }

  return await r.text();
}
