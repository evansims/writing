import { notFound } from "next/navigation";
import ReactMarkdown from "react-markdown";
import Image from "next/image";
import { Metadata } from "next";
import { getContent } from "@/lib/api";
import { getRelativeTimeString } from "@/lib/utils";
import Link from "next/link";
import { ArrowUpLeft, AudioWaveform } from "lucide-react";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import HeaderAnchor from "@/components/HeaderAnchor";
import MainNavigation from "@/components/MainNavigation";

interface ContentPageProps {
  params: {
    slug: string[];
  };
}

export async function generateMetadata({
  params,
}: ContentPageProps): Promise<Metadata> {
  try {
    const resolvedParams = await params;
    const slug = resolvedParams.slug.join("/");
    const content = await getContent(slug);

    return {
      title: `${content.title}`,
      description: content.description || ``,
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
    const resolvedParams = await params;
    const slug = resolvedParams.slug.join("/");
    const content = await getContent(slug);

    if (!content) {
      notFound();
    }

    return (
      <div className="layout-content">
        <MainNavigation label="Main navigation" />

        <header
          aria-label="Content header"
          className="text-muted-foreground mt-8 mb-10 flex flex-row items-center"
        >
          <AudioWaveform size={12} aria-hidden="true" className="mr-1" />

          <div className="flex flex-1 items-center font-semibold">
            <h2>
              <Link href="/" className="text-foreground">
                The Essential Path
              </Link>
            </h2>

            {content.topic && (
              <>
                <ArrowUpLeft
                  size={12}
                  className="mr-1 ml-6"
                  aria-hidden="true"
                />

                <Link
                  href={`/${content.topic.toLowerCase()}`}
                  className="text-muted-foreground font-semibold"
                >
                  {content.topic}
                </Link>
              </>
            )}
          </div>

          <div className="text-sm">
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <div>
                    {content.created && (
                      <time dateTime={content.created}>
                        {getRelativeTimeString(new Date(content.created))}
                      </time>
                    )}
                    {content.updated && content.updated !== content.created && (
                      <time dateTime={content.updated} className="ml-2">
                        (Updated)
                      </time>
                    )}
                  </div>
                </TooltipTrigger>
                <TooltipContent>
                  <p>
                    {content.created &&
                      new Date(content.created).toLocaleDateString("en-US", {
                        year: "numeric",
                        month: "long",
                        day: "numeric",
                      })}
                    {content.updated &&
                      content.updated !== content.created &&
                      " (Updated)"}
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
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

          <div className="content prose prose-shadcn mb-6 max-w-none">
            <ReactMarkdown
              components={{
                h1: ({ node, ...props }) => {
                  const id = props.children
                    ? String(props.children)
                        .toLowerCase()
                        .replace(/\s+/g, "-")
                        .replace(/[^\w-]/g, "")
                    : "";
                  return (
                    <h1 id={id} {...props}>
                      <HeaderAnchor id={id}>{props.children}</HeaderAnchor>
                    </h1>
                  );
                },
                h2: ({ node, ...props }) => {
                  const id = props.children
                    ? String(props.children)
                        .toLowerCase()
                        .replace(/\s+/g, "-")
                        .replace(/[^\w-]/g, "")
                    : "";
                  return (
                    <h2 id={id} {...props}>
                      <HeaderAnchor id={id}>{props.children}</HeaderAnchor>
                    </h2>
                  );
                },
                h3: ({ node, ...props }) => {
                  const id = props.children
                    ? String(props.children)
                        .toLowerCase()
                        .replace(/\s+/g, "-")
                        .replace(/[^\w-]/g, "")
                    : "";
                  return (
                    <h3 id={id} {...props}>
                      <HeaderAnchor id={id}>{props.children}</HeaderAnchor>
                    </h3>
                  );
                },
                h4: ({ node, ...props }) => {
                  const id = props.children
                    ? String(props.children)
                        .toLowerCase()
                        .replace(/\s+/g, "-")
                        .replace(/[^\w-]/g, "")
                    : "";
                  return (
                    <h4 id={id} {...props}>
                      <HeaderAnchor id={id}>{props.children}</HeaderAnchor>
                    </h4>
                  );
                },
                h5: ({ node, ...props }) => {
                  const id = props.children
                    ? String(props.children)
                        .toLowerCase()
                        .replace(/\s+/g, "-")
                        .replace(/[^\w-]/g, "")
                    : "";
                  return (
                    <h5 id={id} {...props}>
                      <HeaderAnchor id={id}>{props.children}</HeaderAnchor>
                    </h5>
                  );
                },
                h6: ({ node, ...props }) => {
                  const id = props.children
                    ? String(props.children)
                        .toLowerCase()
                        .replace(/\s+/g, "-")
                        .replace(/[^\w-]/g, "")
                    : "";
                  return (
                    <h6 id={id} {...props}>
                      <HeaderAnchor id={id}>{props.children}</HeaderAnchor>
                    </h6>
                  );
                },
              }}
            >
              {content.body}
            </ReactMarkdown>
          </div>

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
