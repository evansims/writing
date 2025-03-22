import { notFound } from "next/navigation";
import ReactMarkdown from "react-markdown";
import Image from "next/image";
import { Metadata } from "next";
import { getContent } from "@/lib/api";

interface ContentPageProps {
  params: {
    slug: string;
  };
}

export async function generateMetadata({
  params,
}: ContentPageProps): Promise<Metadata> {
  const { slug } = await params;
  const content = await getContent(slug);
  return {
    title: `${content.title} | Evan Sims`,
    description: content.description,
  };
}

export default async function ContentPage({ params }: ContentPageProps) {
  const { slug } = await params;
  const content = await getContent(slug);

  return (
    <div className="container py-8">
      <article className="prose prose-slate dark:prose-invert max-w-none">
        {/* {content.banner && (
          <div className="relative w-full h-64 md:h-96 mb-8 rounded-lg overflow-hidden">
            <Image
              src={content.banner}
              alt={content.title}
              fill
              className="object-cover"
            />
          </div>
        )} */}

        <h1 className="text-3xl md:text-4xl font-bold mb-4">{content.title}</h1>

        {content.description && (
          <p className="text-xl text-muted-foreground mb-8">
            {content.description}
          </p>
        )}

        <div className="flex flex-wrap gap-2 mb-6">
          {content.tags &&
            content.tags.map((tag) => (
              <span
                key={tag}
                className="px-3 py-1 bg-secondary text-secondary-foreground rounded-full text-sm"
              >
                {tag}
              </span>
            ))}
        </div>

        <div className="content">
          <ReactMarkdown>{content.body}</ReactMarkdown>
        </div>

        <div className="flex items-center justify-start gap-4 text-sm text-muted-foreground mb-8">
          {content.created && (
            <time dateTime={content.created}>
              Published: {new Date(content.created).toLocaleDateString()}
            </time>
          )}
          {content.updated && content.updated !== content.created && (
            <time dateTime={content.updated}>
              Updated: {new Date(content.updated).toLocaleDateString()}
            </time>
          )}
        </div>
      </article>
    </div>
  );
}
