import { notFound } from "next/navigation";

async function getContent(slug: string) {
  const res = await fetch(`/api/content/${slug}`);

  if (!res.ok) {
    if (res.status === 404) return notFound();
    throw new Error("Failed to fetch content");
  }

  return res.json();
}

export default async function ContentPage({ params }) {
  const content = await getContent(params.slug);

  return (
    <article>
      <h1>{content.title}</h1>
      <div dangerouslySetInnerHTML={{ __html: content.body }} />
    </article>
  );
}
