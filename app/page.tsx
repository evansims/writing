import Link from "next/link";
import { getLatestContent } from "@/lib/api";

export default async function Home() {
  const latestContent = await getLatestContent();

  return (
    <div className="@container">
      <div>
        {latestContent.map((item) => (
          <div key={item.slug}>
            <Link
              href={`${item.url}`}
              className="group block flex flex-row px-4 py-2"
            >
              <div className="hover:bg-muted/50 focus:bg-muted/50 w-full rounded-md">
                <p>
                  <span className="font-medium underline-offset-2 group-hover:underline">
                    {item.title}
                  </span>
                  <br />
                  <p className="text-sm">{item.description}</p>
                </p>
              </div>
              <div className="">
                {item.created
                  ? new Date(item.created).toLocaleDateString()
                  : ""}
              </div>
            </Link>
          </div>
        ))}
      </div>
    </div>
  );
}
