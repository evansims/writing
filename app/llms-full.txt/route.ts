import { getLLMsFull } from "@/lib/api";

export async function GET() {
  try {
    const text = await getLLMsFull();

    return new Response(text, {
      headers: {
        "Content-Type": "text/plain; charset=utf-8",
        "Cache-Control": "public, max-age=3600, s-maxage=3600",
      },
    });
  } catch (error) {
    console.error("Error fetching llms-full.txt content:", error);
    return new Response("Error fetching full LLMs content", {
      status: 500,
      headers: {
        "Content-Type": "text/plain; charset=utf-8",
      },
    });
  }
}
