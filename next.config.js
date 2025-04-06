// @ts-check

/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    turbo: {
      // Example: Adding custom aliases for module resolution
      resolveAlias: {
        underscore: "lodash",
      },
      // Example: Extending file resolution extensions
      resolveExtensions: [".mdx", ".tsx", ".ts", ".jsx", ".js", ".json"],
      // Example: Enabling tree shaking
      treeShaking: true,
      // Example: Setting a memory limit for Turbopack (in bytes)
      memoryLimit: 512 * 1024 * 1024, // 512 MB
      // Example: Adding rules for specific file types
      rules: {
        "*.svg": {
          loaders: ["@svgr/webpack"],
          as: "*.js",
        },
      },
    },
  },
  rewrites: async () => {
    return [
      {
        source: "/api/:path*",
        destination:
          process.env.NODE_ENV === "development"
            ? "http://127.0.0.1:5328/api/:path*"
            : "/api/",
      },
    ];
  },
};

module.exports = nextConfig;
