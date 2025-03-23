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
    slug: string[];
  };
}

export async function generateMetadata({
  params,
}: ContentPageProps): Promise<Metadata> {
  try {
    const slug = params.slug.join("/");
    const content = await getContent(slug);

    return {
      title: `${content.title} | Evan Sims`,
      description: content.description || `Article on ${content.title}`,
      openGraph: {
        title: content.title,
        description: content.description,
        type: "article",
        publishedTime: content.created,
        modifiedTime: content.updated,
        tags: content.tags,
      },
    };
  } catch (error) {
    return {
      title: "Content Not Found | Evan Sims",
      description: "The requested content could not be found",
    };
  }
}

export default async function ContentPage({ params }: ContentPageProps) {
  try {
    const slug = params.slug.join("/");
    const content = await getContent(slug);

    if (!content) {
      notFound();
    }

    return (
      <div className="layout-content">
        <nav aria-label="Main navigation" className="mt-8 mb-10">
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
        </nav>

        <header
          aria-label="Content header"
          className="text-muted-foreground mt-8 mb-10 flex flex-row items-center"
        >
          <ArrowUp size={12} className="mr-1" aria-hidden="true" />

          <div className="flex flex-1 items-center font-semibold">
            <h2>
              <Link href="/" className="text-foreground">
                The Essential Path
              </Link>
            </h2>

            <ArrowUpLeft size={12} className="mr-1 ml-6" aria-hidden="true" />

            <Link
              href="/mindset"
              className="text-muted-foreground font-semibold"
            >
              Mindset
            </Link>
          </div>

          <div className="text-sm">
            {content.created && (
              <time
                dateTime={content.created}
                title={new Date(content.created).toLocaleDateString("en-US", {
                  month: "long",
                  day: "numeric",
                  year: "numeric",
                })}
              >
                {getRelativeTimeString(new Date(content.created))}
              </time>
            )}
            {content.updated && content.updated !== content.created && (
              <time
                dateTime={content.updated}
                className="ml-2"
                title={new Date(content.updated).toLocaleDateString("en-US", {
                  month: "long",
                  day: "numeric",
                  year: "numeric",
                })}
              >
                (Updated)
              </time>
            )}
          </div>
        </header>

        <article>
          {content.banner ? (
            <div className="-mx-[var(--page-padding-inline)] mb-10 w-[calc(100%+var(--page-padding-left)+var(--page-padding-right))]">
              {/* <Image
                src={content.banner}
                alt={`Cover image for ${content.title}`}
                width={1200}
                height={630}
                priority
                className="h-96 w-full rounded-lg object-cover"
              /> */}
            </div>
          ) : (
            <div
              className="border-muted -mx-[var(--page-padding-inline)] mb-10 h-96 w-[calc(100%+var(--page-padding-left)+var(--page-padding-right))] rounded-lg border-1"
              aria-hidden="true"
            />
          )}

          <h1 className="mb-4 text-3xl font-semibold md:text-4xl">
            {content.title}
          </h1>

          {content.description && (
            <p className="text-muted-foreground mb-8">{content.description}</p>
          )}

          <div
            className="content prose prose-shadcn mb-6 max-w-none"
            // Replace dangerouslySetInnerHTML with ReactMarkdown when content is in markdown format
            dangerouslySetInnerHTML={{ __html: content.body }}
          />

          {content.tags && content.tags.length > 0 && (
            <footer>
              <div
                className="mb-6 flex flex-wrap gap-2"
                aria-label="Article tags"
              >
                {content.tags.map((tag) => (
                  <span
                    key={tag}
                    className="bg-secondary text-secondary-foreground rounded-full px-3 py-1 text-sm"
                  >
                    {tag}
                  </span>
                ))}
              </div>
            </footer>
          )}
        </article>
      </div>
    );
  } catch (error) {
    notFound();
  }
}
