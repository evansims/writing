import { NextRequest } from "next/server";

/**
 * Route handler for serving llms.txt content from the backend API
 * Uses Server Component for optimal performance and SEO
 */
export async function GET(request: NextRequest) {
  try {
    // Fetch content from the backend API
    const response = await fetch(
      `${process.env.NEXT_PUBLIC_API_URL || ""}/api/llms.txt`,
      {
        next: { revalidate: 3600 }, // Cache for 1 hour
      }
    );

    if (!response.ok) {
      throw new Error(`API responded with status: ${response.status}`);
    }

    // Get the text content
    const text = await response.text();

    // Return the content with the correct content type
    return new Response(text, {
      headers: {
        "Content-Type": "text/plain; charset=utf-8",
        "Cache-Control": "public, max-age=3600, s-maxage=3600",
      },
    });
  } catch (error) {
    console.error("Error fetching llms.txt content:", error);
    return new Response("Error fetching LLMs content", {
      status: 500,
      headers: {
        "Content-Type": "text/plain; charset=utf-8",
      },
    });
  }
}
