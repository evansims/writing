"use client";

import dynamic from "next/dynamic";

const WaterStream = dynamic(() => import("@/components/art/TriangleGravity"), {
  ssr: false,
});

export default function ParticleNetworkWrapper() {
  return <WaterStream />;
}
