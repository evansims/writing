import Link from "next/link";
import { getLatestContent } from "@/lib/api";
import { AudioWaveform, Bot, Rss } from "lucide-react";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { formatDate } from "@/lib/utils";
import { Suspense } from "react";
import MainNavigation from "@/components/MainNavigation";
import TopicLink from "@/components/TopicLink";

function LatestContentSkeleton() {
  return (
    <div className="animate-pulse">
      <div className="mb-6 grid grid-cols-2 gap-6">
        {[1, 2].map((i) => (
          <div key={i} className="bg-card/50 h-40 rounded-lg border-1"></div>
        ))}
      </div>
      <div className="mb-6 grid grid-cols-3 gap-6">
        {[1, 2, 3].map((i) => (
          <div key={i} className="bg-card/50 h-32 rounded-lg border-1"></div>
        ))}
      </div>
    </div>
  );
}

export default async function Home() {
  return (
    <div className="layout-content">
      <a
        href="#main-content"
        className="focus:bg-background focus:text-foreground sr-only focus:not-sr-only focus:absolute focus:z-10 focus:p-4"
      >
        Skip to main content
      </a>

      <MainNavigation />

      <header className="mt-8 mb-10">
        <div className="flex items-center">
          <AudioWaveform
            size={16}
            aria-hidden="true"
            className="mr-2 inline-block"
          />
          <h1 className="inline-block text-2xl font-semibold">
            The Essential Path
          </h1>
        </div>
        <p className="text-muted-foreground">
          Navigating life's complexity with intentional simplicity — by{" "}
          <Link href="/about" className="text-foreground">
            Evan Sims
          </Link>
          .
        </p>
      </header>

      <main id="main-content">
        <div className="mb-4 flex items-center justify-between">
          <h2 className="flex-1 font-semibold">Latest</h2>
          <div className="flex items-center">
            <span className="text-muted-foreground hover:text-foreground focus-within:text-foreground mr-2">
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <Link
                      href="/llms.xmtxtl"
                      className="block p-2"
                      aria-label="Content structured for Large Language Models"
                    >
                      <Bot size={16} aria-hidden="true" />
                    </Link>
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>Content structured for Large Language Models</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </span>
            <span className="text-muted-foreground hover:text-foreground focus-within:text-foreground">
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <Link
                      href="/rss.xml"
                      className="block p-2"
                      aria-label="RSS Feed"
                    >
                      <Rss size={16} aria-hidden="true" />
                    </Link>
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>RSS Feed</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </span>
          </div>
        </div>

        <Suspense fallback={<LatestContentSkeleton />}>
          <LatestContent />
        </Suspense>
      </main>

      <section className="mt-12 mb-8 border-t pt-8">
        <div className="max-w-md">
          <h2 className="mb-3 text-xl font-semibold" id="newsletter">
            Stay Updated
          </h2>
          <p className="text-muted-foreground mb-4">
            Get new content delivered directly to your inbox.
          </p>

          <form
            className="flex flex-col gap-3 sm:flex-row"
            aria-labelledby="newsletter"
          >
            <label htmlFor="email-input" className="sr-only">
              Your email address
            </label>
            <input
              id="email-input"
              type="email"
              placeholder="Your email address"
              className="bg-background focus:ring-ring flex-1 rounded-lg border px-4 py-2 text-sm focus:ring-2 focus:outline-none"
              required
              aria-required="true"
            />
            <button
              type="submit"
              className="bg-primary text-primary-foreground hover:bg-primary/90 rounded-lg px-4 py-2 text-sm font-medium transition-colors"
            >
              Subscribe
            </button>
          </form>
        </div>
      </section>
    </div>
  );
}

async function LatestContent() {
  try {
    const latestContent = await getLatestContent(20, ["article"]);

    if (!latestContent || latestContent.length === 0) {
      return <p>No content available at the moment.</p>;
    }

    return (
      <>
        <div className="mb-6 grid grid-cols-1 gap-6 sm:grid-cols-2">
          {latestContent
            .filter((item) => item.created)
            .slice(0, 2)
            .map((item) => (
              <Link
                key={item.slug}
                href={`${item.url}`}
                className="group block"
              >
                <article className="bg-card group-hover:bg-muted/25 group-focus:bg-muted/25 focus-within:bg-muted/25 h-full rounded-lg border-1 p-6 transition-colors">
                  {item.topic && (
                    <TopicLink
                      topic={item.topic}
                      className="text-muted-foreground mb-2 block text-sm"
                    />
                  )}
                  <h3 className="text-xl font-semibold">{item.title}</h3>
                  {item.description && (
                    <p className="text-muted-foreground mt-2 line-clamp-2">
                      {item.description}
                    </p>
                  )}
                </article>
              </Link>
            ))}
        </div>

        <div className="mb-6 grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3">
          {latestContent
            .filter((item) => item.created)
            .slice(2, 5)
            .map((item) => (
              <Link
                key={item.slug}
                href={`${item.url}`}
                className="group block"
              >
                <article className="bg-card group-hover:bg-muted/25 group-focus:bg-muted/25 focus-within:bg-muted/25 h-full rounded-lg border-1 p-6 transition-colors">
                  {item.topic && (
                    <TopicLink
                      topic={item.topic}
                      className="text-muted-foreground mb-2 block text-sm"
                    />
                  )}
                  <h3 className="text-lg font-semibold">{item.title}</h3>
                </article>
              </Link>
            ))}
        </div>

        <aside aria-labelledby="previously-heading">
          <h2
            id="previously-heading"
            className="text-muted-foreground mt-8 mb-2"
          >
            Previously
          </h2>
          <ul className="stacker">
            {latestContent
              .filter((item) => item.created)
              .slice(0, 15)
              .map((item) => (
                <li key={item.slug}>
                  <Link href={`${item.url}`} className="block">
                    <article className="flex items-center py-2">
                      <time
                        dateTime={item.created}
                        className="text-muted-foreground pr-4 text-sm"
                      >
                        {item.created ? formatDate(new Date(item.created)) : ""}
                      </time>

                      <h3 className="flex-1 font-semibold">{item.title}</h3>

                      {item.topic && (
                        <TopicLink
                          topic={item.topic}
                          className="text-muted-foreground text-sm"
                        />
                      )}
                    </article>
                  </Link>
                </li>
              ))}
          </ul>
        </aside>
      </>
    );
  } catch (error) {
    console.error("Failed to load content:", error);
    return <p>Failed to load content. Please try again later.</p>;
  }
}
