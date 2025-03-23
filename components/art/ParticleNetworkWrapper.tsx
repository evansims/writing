"use client";

import dynamic from "next/dynamic";

const MindfulParticles = dynamic(
  () => import("@/components/art/TriangleGravity"),
  {
    ssr: false,
  },
);

export default function ParticleNetworkWrapper() {
  return <MindfulParticles />;
}
