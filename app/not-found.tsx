import Link from "next/link";

export default function NotFound() {
  return (
    <div className="error-container">
      <h1>404 - Content Not Found</h1>
      <p>The requested resource could not be located</p>
      <Link href="/">Return to Home</Link>
    </div>
  );
}
