import { notFound } from "next/navigation";
import ReactMarkdown from "react-markdown";
import Image from "next/image";
import { Metadata } from "next";
import { getContent } from "@/lib/api";
import { getRelativeTimeString } from "@/lib/utils";
import Link from "next/link";
import {
  ArrowLeft,
  ArrowUp,
  ArrowUpLeft,
  ChevronRight,
  CornerLeftUp,
} from "lucide-react";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";

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
    <div className="layout-content">
      <div className="mt-8 mb-10">
        <ul className="text-muted-foreground flex space-x-6">
          <li>
            <Link href="/about">About</Link>
          </li>
          <li>
            <Link href="/audio">Audio</Link>
          </li>
          <li>
            <Link href="/store">Store</Link>
          </li>
          <li>
            <Link href="/now">Now</Link>
          </li>
        </ul>
      </div>

      <div className="text-muted-foreground mt-8 mb-10 flex flex-row items-center">
        <ArrowUp size={12} className="mr-1" aria-hidden="true" />

        <div className="flex flex-1 items-center font-semibold">
          <h1>
            <Link href="/" className="text-foreground">
              The Essential Path
            </Link>
          </h1>

          <ArrowUpLeft size={12} className="mr-1 ml-6" aria-hidden="true" />

          <Link href="/mindset" className="text-muted-foreground font-semibold">
            Mindset
          </Link>
        </div>

        <div className="text-sm">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger>
                {content.created && (
                  <time dateTime={content.created}>
                    {new Date(content.created).toLocaleDateString("en-US", {
                      month: "long",
                      day: "numeric",
                      year: "numeric",
                    })}
                  </time>
                )}
                {content.updated && content.updated !== content.created && (
                  <time dateTime={content.updated}>
                    {new Date(content.updated).toLocaleDateString("en-US", {
                      month: "long",
                      day: "numeric",
                      year: "numeric",
                    })}
                  </time>
                )}
              </TooltipTrigger>
              <TooltipContent>
                <p>
                  {content.updated
                    ? `Updated ${getRelativeTimeString(new Date(content.updated))}`
                    : content.created
                      ? `Published ${getRelativeTimeString(new Date(content.created))}`
                      : "Last updated"}
                </p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>
      </div>

      <article>
        <div className="border-muted -mx-[var(--page-padding-inline)] mb-10 h-96 w-[calc(100%+var(--page-padding-left)+var(--page-padding-right))] rounded-lg border-1"></div>

        <h1 className="mb-4 text-3xl font-semibold md:text-4xl">
          {content.title}
        </h1>

        {content.description && (
          <p className="text-muted-foreground mb-8">{content.description}</p>
        )}

        <div
          className="content prose prose-shadcn mb-6 max-w-none"
          dangerouslySetInnerHTML={{ __html: content.body }}
        />

        <div className="mb-6 flex flex-wrap gap-2">
          {content.tags &&
            content.tags.map((tag) => (
              <span
                key={tag}
                className="bg-secondary text-secondary-foreground rounded-full px-3 py-1 text-sm"
              >
                {tag}
              </span>
            ))}
        </div>
      </article>
    </div>
  );
}
