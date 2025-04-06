import { getLLMs } from "@/lib/api";

export async function GET() {
  try {
    const text = await getLLMs();

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
