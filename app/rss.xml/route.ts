import { NextRequest } from "next/server";

/**
 * Route handler for serving RSS feed content from the backend API
 * Uses Server Component for optimal performance and SEO
 */
export async function GET(request: NextRequest) {
  try {
    // Fetch content from the backend API
    const response = await fetch(
      `${process.env.NEXT_PUBLIC_API_URL || ""}/api/rss/blog`,
      {
        next: { revalidate: 3600 }, // Cache for 1 hour
      }
    );

    if (!response.ok) {
      throw new Error(`API responded with status: ${response.status}`);
    }

    // Get the XML content
    const xml = await response.text();

    // Return the content with the correct content type
    return new Response(xml, {
      headers: {
        "Content-Type": "application/xml",
        "Cache-Control": "public, max-age=3600, s-maxage=3600",
      },
    });
  } catch (error) {
    console.error("Error fetching RSS feed content:", error);

    // Create a basic valid XML response on error
    const fallbackXml = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Evan Sims</title>
    <link>${process.env.NEXT_PUBLIC_API_URL || "https://evansims.com"}</link>
    <description>Content temporarily unavailable</description>
  </channel>
</rss>`;

    return new Response(fallbackXml, {
      status: 500,
      headers: {
        "Content-Type": "application/xml",
      },
    });
  }
}
