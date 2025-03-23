"use client";

import React, { useRef, useEffect } from "react";

interface ParticleProperties {
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  opacity: number;
  turbulence: number;
}

class Particle implements ParticleProperties {
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  opacity: number;
  turbulence: number;
  initialY: number;

  constructor(canvasWidth: number, canvasHeight: number) {
    // Start particles from the top, distributed across width
    this.x = Math.random() * canvasWidth;
    this.y = Math.random() * canvasHeight * 0.3; // Start in top third
    this.initialY = this.y;

    // Initial velocity - mainly downward with slight horizontal variation
    this.vx = (Math.random() - 0.5) * 0.5;
    this.vy = Math.random() * 1 + 0.5;

    // Varied sizes for depth effect
    this.size = Math.random() * 2 + 0.5;

    // Random opacity for water effect
    this.opacity = Math.random() * 0.3 + 0.1;

    // Individual turbulence factor
    this.turbulence = Math.random() * 0.1;
  }

  update(canvasWidth: number, canvasHeight: number, time: number): void {
    // Add sinusoidal motion for flowing effect
    const flowOffset =
      Math.sin(time * 0.001 + this.initialY * 0.1) * this.turbulence;
    this.vx += flowOffset;

    // Apply velocity
    this.x += this.vx;
    this.y += this.vy;

    // Gradually stabilize horizontal velocity
    this.vx *= 0.99;

    // Add some natural turbulence
    if (Math.random() < 0.05) {
      this.vx += (Math.random() - 0.5) * 0.3;
      this.vy += (Math.random() - 0.5) * 0.1;
    }

    // Keep vertical speed within bounds
    this.vy = Math.max(0.5, Math.min(2, this.vy));

    // Reset particle if it goes off screen
    if (this.y > canvasHeight) {
      this.reset(canvasWidth);
    }

    // Wrap horizontally
    if (this.x < 0) this.x = canvasWidth;
    if (this.x > canvasWidth) this.x = 0;

    // Subtle opacity variation
    this.opacity += (Math.random() - 0.5) * 0.05;
    this.opacity = Math.max(0.1, Math.min(0.4, this.opacity));
  }

  reset(canvasWidth: number): void {
    this.x = Math.random() * canvasWidth;
    this.y = 0;
    this.initialY = this.y;
    this.vx = (Math.random() - 0.5) * 0.5;
    this.vy = Math.random() * 1 + 0.5;
  }

  draw(ctx: CanvasRenderingContext2D, particles: Particle[]): void {
    // Draw connections to nearby particles
    for (let particle of particles) {
      const dx = particle.x - this.x;
      const dy = particle.y - this.y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      // Only connect to particles that are close and flowing in a similar direction
      if (distance < 50 && Math.abs(this.y - particle.y) < 30) {
        const opacity =
          (1 - distance / 50) * 0.15 * (1 - Math.abs(this.y - particle.y) / 30);
        ctx.beginPath();
        ctx.strokeStyle = `rgba(255, 255, 255, ${opacity})`;
        ctx.lineWidth = 0.5;
        ctx.moveTo(this.x, this.y);
        ctx.lineTo(particle.x, particle.y);
        ctx.stroke();
      }
    }

    // Draw particle
    ctx.fillStyle = `rgba(255, 255, 255, ${this.opacity})`;
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
    ctx.fill();
  }
}

const WaterStream = () => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const timeRef = useRef<number>(0);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Make canvas fill parent div
    const resizeCanvas = () => {
      const parent = canvas.parentElement;
      if (parent) {
        canvas.width = parent.clientWidth;
        canvas.height = parent.clientHeight;
      }
    };
    resizeCanvas();

    // Create particles
    const particles: Particle[] = [];
    const particleCount = Math.min(
      200,
      Math.floor((canvas.width * canvas.height) / 5000),
    );

    for (let i = 0; i < particleCount; i++) {
      particles.push(new Particle(canvas.width, canvas.height));
    }

    // Animation loop
    const animate = (timestamp: number) => {
      timeRef.current = timestamp;

      ctx.fillStyle = "rgba(0, 0, 0, 0.1)";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      for (let particle of particles) {
        particle.update(canvas.width, canvas.height, timestamp);
        particle.draw(ctx, particles);
      }

      requestAnimationFrame(animate);
    };

    // Start animation and add event listener
    animate(0);
    window.addEventListener("resize", resizeCanvas);

    // Cleanup
    return () => {
      window.removeEventListener("resize", resizeCanvas);
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      style={{
        width: "100%",
        height: "100%",
        background: "black",
        position: "absolute",
        top: 0,
        left: 0,
      }}
    />
  );
};

export default WaterStream;
