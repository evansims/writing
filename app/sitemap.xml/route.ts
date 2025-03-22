import { NextRequest } from "next/server";

/**
 * Route handler for serving sitemap.xml content from the backend API
 * Uses Server Component for optimal performance and SEO
 */
export async function GET(request: NextRequest) {
  try {
    // Fetch content from the backend API
    const response = await fetch(
      `${process.env.NEXT_PUBLIC_API_URL || ""}/api/sitemap.xml`,
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
    console.error("Error fetching sitemap.xml content:", error);

    // Create a basic valid XML response on error
    const fallbackXml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>${process.env.NEXT_PUBLIC_API_URL || "https://evansims.com"}</loc>
    <lastmod>${new Date().toISOString().split("T")[0]}</lastmod>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
</urlset>`;

    return new Response(fallbackXml, {
      status: 500,
      headers: {
        "Content-Type": "application/xml",
      },
    });
  }
}
