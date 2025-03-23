"use client";

import React, { useRef, useEffect } from "react";

// Define interfaces for our types
interface ParticleProperties {
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  speedFactor: number;
}

interface Attractor {
  x: number;
  y: number;
  strength: number;
}

// Define the Particle class outside of the component to improve readability
class Particle implements ParticleProperties {
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  speedFactor: number;

  constructor(canvasWidth: number, canvasHeight: number) {
    // Random starting position
    this.x = Math.random() * canvasWidth;
    this.y = Math.random() * canvasHeight;

    // Random initial velocity
    this.vx = (Math.random() - 0.5) * 0.5;
    this.vy = (Math.random() - 0.5) * 0.5;

    // Particle size (small)
    this.size = Math.random() * 1.5 + 0.5;

    // Speed factor
    this.speedFactor = Math.random() * 0.2 + 0.05;
  }

  // Update particle position
  update(
    attractors: Attractor[],
    canvasWidth: number,
    canvasHeight: number,
  ): void {
    // Calculate forces from attractors (triangle points)
    for (let attractor of attractors) {
      // Vector from particle to attractor
      const dx = attractor.x - this.x;
      const dy = attractor.y - this.y;

      // Distance to attractor
      const distance = Math.sqrt(dx * dx + dy * dy);

      // Only apply force if particle is not too close (prevents infinite acceleration)
      if (distance > 5) {
        // Force is stronger as particles get closer
        const force = attractor.strength / (distance * distance);

        // Apply force to velocity
        this.vx += (dx / distance) * force * this.speedFactor;
        this.vy += (dy / distance) * force * this.speedFactor;
      }
    }

    // Apply slight drag to prevent infinite acceleration
    this.vx *= 0.99;
    this.vy *= 0.99;

    // Update position
    this.x += this.vx;
    this.y += this.vy;

    // Wrap around screen edges
    if (this.x < 0) this.x = canvasWidth;
    if (this.x > canvasWidth) this.x = 0;
    if (this.y < 0) this.y = canvasHeight;
    if (this.y > canvasHeight) this.y = 0;
  }

  // Draw particle
  draw(ctx: CanvasRenderingContext2D): void {
    ctx.fillStyle = "#ffffff"; // Pure white
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
    ctx.fill();
  }
}

const ParticleBlackHole = () => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Set canvas to full window size
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    // Create particles
    const particles: Particle[] = [];
    const particleCount = 7000; // Adjust based on performance

    for (let i = 0; i < particleCount; i++) {
      particles.push(new Particle(canvas.width, canvas.height));
    }

    // Triangle attractor points
    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    const triangleSize = Math.min(canvas.width, canvas.height) * 0.15;

    const attractors: Attractor[] = [
      // Triangle points act as attractors
      {
        x: centerX,
        y: centerY - triangleSize,
        strength: 20,
      },
      {
        x: centerX - triangleSize * 0.866, // cos(60°) = 0.866
        y: centerY + triangleSize * 0.5, // sin(60°) = 0.5
        strength: 20,
      },
      {
        x: centerX + triangleSize * 0.866,
        y: centerY + triangleSize * 0.5,
        strength: 20,
      },
    ];

    // Animation loop
    const animate = () => {
      // Clear canvas with semi-transparent black for trail effect
      ctx.fillStyle = "rgba(0, 0, 0, 0.05)";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Update and draw particles
      for (let particle of particles) {
        particle.update(attractors, canvas.width, canvas.height);
        particle.draw(ctx);
      }

      requestAnimationFrame(animate);
    };

    // Start animation
    animate();

    // Handle window resize
    const handleResize = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    };

    window.addEventListener("resize", handleResize);

    // Cleanup
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      style={{
        background: "#000",
      }}
    />
  );
};

export default ParticleBlackHole;
