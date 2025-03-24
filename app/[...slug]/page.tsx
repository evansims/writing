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
import TableOfContents from "@/components/TableOfContents";
import GeometricScene from "@/components/art/TriangleGravity";
import ContentTextToSpeech from "@/components/ContentTextToSpeech";
import { ReactNode } from "react";

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

// Helper function to process markdown content into sections
function processContentSections(content: string): {
  intro: string;
  sections: Array<{ heading: string; content: string; id: string }>;
} {
  const lines = content.split("\n");
  const intro: string[] = [];
  const sections: Array<{ heading: string; content: string; id: string }> = [];

  let currentSection: string[] = [];
  let currentHeading = "";
  let currentId = "";

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    if (line.startsWith("## ")) {
      // If we find an H2, store previous content and start a new section
      if (currentHeading) {
        sections.push({
          heading: currentHeading,
          content: currentSection.join("\n"),
          id: currentId,
        });
      } else if (currentSection.length > 0 && intro.length === 0) {
        // This is the introductory content before any h2
        intro.push(...currentSection);
      }

      currentHeading = line.substring(3);
      currentId = currentHeading
        .toLowerCase()
        .replace(/\s+/g, "-")
        .replace(/[^\w-]/g, "");
      currentSection = [line];
    } else {
      // If we haven't encountered any H2 yet, this is intro
      if (!currentHeading && sections.length === 0) {
        intro.push(line);
      } else {
        currentSection.push(line);
      }
    }
  }

  // Add the last section
  if (currentHeading) {
    sections.push({
      heading: currentHeading,
      content: currentSection.join("\n"),
      id: currentId,
    });
  } else if (currentSection.length > 0 && intro.length === 0) {
    intro.push(...currentSection);
  }

  return { intro: intro.join("\n"), sections };
}

export default async function ContentPage({ params }: ContentPageProps) {
  try {
    const resolvedParams = await params;
    const slug = resolvedParams.slug.join("/");
    const content = await getContent(slug);

    if (!content) {
      notFound();
    }

    // Process the content into sections
    const { intro, sections } = processContentSections(content.body);

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

        {/* Banner moved outside article to full width */}
        {content.banner ? (
          <div className="mx-auto mb-10 w-full max-w-[var(--max-content-width)]">
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
          <div className="border-muted relative mx-auto mb-10 h-96 w-full max-w-[var(--max-content-width)] overflow-hidden rounded-lg border-1">
            <div className="absolute inset-0">
              <GeometricScene
                seed={content.title}
                complexity={50}
                rotationSpeed={0.5}
              />
            </div>
          </div>
        )}

        <div className="content-layout mx-auto max-w-[var(--max-content-width)]">
          <article className="main-content">
            <h1 className="mb-4 text-3xl font-semibold md:text-4xl">
              {content.title}
            </h1>

            {content.description && content.description !== "None" && (
              <p className="text-muted-foreground mb-8">
                {content.description}
              </p>
            )}

            {/* Text-to-Speech feature */}
            <ContentTextToSpeech
              content={content.body}
              title={content.title}
              slug={slug}
            />

            <div className="content prose prose-shadcn mb-6 max-w-none">
              {/* Introduction section */}
              {intro && (
                <div className="intro-section" data-section-type="intro">
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
                          </h6>
                        );
                      },
                    }}
                  >
                    {intro}
                  </ReactMarkdown>
                </div>
              )}

              {/* Content sections */}
              {sections.map((section, index) => (
                <div
                  key={`section-${index}`}
                  className="content-section"
                  id={section.id}
                  data-section-type="content"
                  data-section-id={section.id}
                >
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
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
                            <HeaderAnchor id={id}>
                              {props.children}
                            </HeaderAnchor>
                          </h6>
                        );
                      },
                    }}
                  >
                    {section.content}
                  </ReactMarkdown>
                </div>
              ))}
            </div>

            {content.reading && content.reading.length > 0 && (
              <div className="border-muted my-8 border-t pt-6">
                <h3 className="mb-4 text-lg font-semibold">Further Reading</h3>
                <ul className="space-y-3">
                  {content.reading.map((item, index) => (
                    <li key={index} className="flex flex-col">
                      <a
                        href={item.url}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-foreground font-medium hover:underline"
                      >
                        {item.title}
                      </a>
                      <span className="text-muted-foreground text-sm">
                        by {item.author}
                      </span>
                    </li>
                  ))}
                </ul>
              </div>
            )}

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

          <div className="toc-container">
            <TableOfContents />
          </div>
        </div>
      </div>
    );
  } catch (error) {
    notFound();
  }
}
