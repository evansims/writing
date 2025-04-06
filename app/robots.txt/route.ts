import { NextRequest } from "next/server";
import { getPublicUrl } from "@/lib/api";

/**
 * Route handler for serving robots.txt
 * Uses Server Component for optimal performance and SEO
 */
export async function GET(request: NextRequest) {
  const baseUrl = getPublicUrl();

  // Create robots.txt content
  const robotsTxt = `# https://www.robotstxt.org/robotstxt.html
User-agent: *
Allow: /

# Sitemaps
Sitemap: ${baseUrl}/sitemap.xml

# LLMs Text files
Allow: /llms.txt
Allow: /llms-full.txt
`;

  return new Response(robotsTxt, {
    headers: {
      "Content-Type": "text/plain",
      "Cache-Control": "public, max-age=3600, s-maxage=3600",
    },
  });
}
