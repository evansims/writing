import Link from "next/link";
import { getLatestContent } from "@/lib/api";
import { Bot, Rss } from "lucide-react";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";

export default async function Home() {
  const latestContent = await getLatestContent();

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

      <div className="mt-8 mb-10">
        <h1 className="text-2xl font-semibold">The Essential Path</h1>
        <p className="text-muted-foreground">
          Engineering, philosophy, productivity, and the art of living by{" "}
          <Link href="/about" className="text-foreground">
            Evan Sims
          </Link>
          .
        </p>
      </div>

      <div className="mb-4 flex items-center justify-between">
        <h2 className="flex-1 font-semibold">Latest</h2>
        <p className="text-muted-foreground hover:text-foreground focus-within:text-foreground mr-2">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger>
                <Link href="/llms.xmtxtl" className="block p-2">
                  <Bot size={16} />
                </Link>
              </TooltipTrigger>
              <TooltipContent>
                <p>Content structured for Large Language Models</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </p>
        <p className="text-muted-foreground hover:text-foreground focus-within:text-foreground">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger>
                <Link href="/rss.xml" className="block p-2">
                  <Rss size={16} />
                </Link>
              </TooltipTrigger>
              <TooltipContent>
                <p>RSS Feed</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </p>
      </div>

      <div className="mb-6 grid grid-cols-2 gap-6">
        {latestContent
          .filter((item) => item.created)
          .slice(0, 2)
          .map((item) => (
            <Link key={item.slug} href={`${item.url}`} className="group block">
              <article className="bg-card group-hover:bg-muted/25 group-focus:bg-muted/25 h-full rounded-lg border-1 p-6 transition-colors">
                <div className="text-muted-foreground mb-2 text-sm">Topic</div>
                <h3 className="text-xl font-semibold">{item.title}</h3>
              </article>
            </Link>
          ))}
      </div>

      <div className="mb-6 grid grid-cols-3 gap-6">
        {latestContent
          .filter((item) => item.created)
          .slice(2, 5)
          .map((item) => (
            <Link key={item.slug} href={`${item.url}`} className="group block">
              <article className="bg-card group-hover:bg-muted/25 group-focus:bg-muted/25 h-full rounded-lg border-1 p-6 transition-colors">
                <div className="text-muted-foreground mb-2 text-sm">Topic</div>
                <h3 className="text-lg font-semibold">{item.title}</h3>
              </article>
            </Link>
          ))}
      </div>

      <aside>
        <h2 className="text-muted-foreground mt-8 mb-2">Previously</h2>
        <ul className="stacker">
          {latestContent
            .filter((item) => item.created)
            .slice(0, 15)
            .map((item) => (
              <li key={item.slug}>
                <Link href={`${item.url}`} className="block">
                  <article className="flex items-center py-2">
                    <div className="text-muted-foreground pr-4 text-sm">
                      {`${new Date(item.created ?? "").getFullYear()} / ${new Date(item.created ?? "").getMonth() + 1}`}
                    </div>

                    <h3 className="flex-1 font-semibold">{item.title}</h3>

                    <div className="text-muted-foreground text-sm">Topic</div>
                  </article>
                </Link>
              </li>
            ))}
        </ul>
      </aside>

      <section className="mt-12 mb-8 border-t pt-8">
        <div className="max-w-md">
          <h2 className="mb-3 text-xl font-semibold">Stay Updated</h2>
          <p className="text-muted-foreground mb-4">
            Get new content delivered directly to your inbox.
          </p>

          <form className="flex flex-col gap-3 sm:flex-row">
            <input
              type="email"
              placeholder="Your email address"
              className="bg-background focus:ring-ring flex-1 rounded-lg border px-4 py-2 text-sm focus:ring-2 focus:outline-none"
              required
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
