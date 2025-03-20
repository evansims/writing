import Link from "next/link";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { getLatestContent } from "@/lib/api";

export default async function Home() {
  const latestContent = await getLatestContent();

  return (
    <div className="container py-12">
      <section className="mb-12">
        <h1 className="text-4xl font-bold mb-6">Evan Sims</h1>
        <p className="text-xl mb-4">
          Welcome to my personal site where I share my thoughts on technology,
          engineering, and more.
        </p>
      </section>

      <section className="mb-12">
        <h2 className="text-2xl font-bold mb-6">Latest Content</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {latestContent.map((item) => (
            <Card key={item.slug} className="h-full">
              <CardHeader>
                <CardTitle className="text-xl">
                  <Link href={`/${item.slug}`} className="hover:underline">
                    {item.title}
                  </Link>
                </CardTitle>
                <CardDescription>
                  {new Date(item.created).toLocaleDateString()}
                </CardDescription>
              </CardHeader>
              <CardContent>
                <p>{item.description}</p>
              </CardContent>
            </Card>
          ))}
        </div>
      </section>
    </div>
  );
}
