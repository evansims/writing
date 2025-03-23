"use client";

import React, { useRef, useMemo } from "react";
import { Canvas, useFrame, useThree, ThreeElements } from "@react-three/fiber";
import { Vector3 } from "three";
import { EdgesGeometry, LineSegments, BufferGeometry } from "three";
import * as THREE from "three";
import { extend } from "@react-three/fiber";

// Extend Three.js elements to make them available as JSX elements
extend({
  EdgesGeometry,
  LineSegments,
  BufferGeometry,
  ...Object.keys(THREE).reduce((acc: any, key) => {
    const value = (THREE as any)[key];
    if (value && value.prototype instanceof THREE.BufferGeometry) {
      acc[key] = value;
    }
    return acc;
  }, {}),
});

type ThreeGeometryElements = {
  [K in GeometryType as `${K}Geometry`]: ThreeElements["meshBasicMaterial"];
} & {
  group: ThreeElements["group"];
  mesh: ThreeElements["mesh"];
  lineSegments: ThreeElements["mesh"];
  edgesGeometry: ThreeElements["bufferGeometry"];
  lineBasicMaterial: ThreeElements["meshBasicMaterial"];
  meshBasicMaterial: ThreeElements["meshBasicMaterial"];
  ambientLight: ThreeElements["ambientLight"];
  pointLight: ThreeElements["pointLight"];
};

declare global {
  namespace JSX {
    interface IntrinsicElements extends ThreeGeometryElements {}
  }
}

// Add RootState type for useFrame
type RootState = {
  clock: THREE.Clock;
  camera: THREE.Camera;
  gl: THREE.WebGLRenderer;
};

type GeometryType =
  | "octahedron"
  | "tetrahedron"
  | "box"
  | "icosahedron"
  | "dodecahedron"
  | "torus"
  | "ring"
  | "plane"
  | "cylinder"
  | "cone"
  | "capsule";

// Add types for black holes with enhanced parameters
interface BlackHoleStyle {
  pulseSpeed: number;
  pulseIntensity: number;
  teleportThreshold: number;
  exitSpread: number;
  colorShift: number;
}

interface BlackHole {
  position: Vector3;
  strength: number;
  radius: number;
  connectedTo?: BlackHole;
  phase: number;
  style: BlackHoleStyle;
}

// Add more pattern types based on different characteristics
const patterns = [
  "fibonacci",
  "vortex",
  "lattice",
  "quantum",
  "chaos",
  "spiral",
  "wave",
  "helix",
  "mandala",
  "starburst",
  "nebula",
  "crystal",
  "dna",
  "circuit",
  "origami",
  "constellation",
  "matrix",
  "pulse",
  "ripple",
  "fractal",
  "infinity",
  "balance",
  "mindflow",
  "recursion",
  "binary",
  "meditation",
  "breath",
  "harmony",
  "paradox",
  "emergence",
  "wisdom",
  "dialectic",
  "entropy",
  "causality",
  "synthesis",
] as const;

type PatternType = (typeof patterns)[number];

// Add color themes that will be selected based on the seed
interface ColorTheme {
  primary: string;
  secondary: string;
  accent: string;
  background: string;
}

const colorThemes: ColorTheme[] = [
  {
    primary: "#ffffff", // Pure white
    secondary: "#cccccc", // Light gray
    accent: "#666666", // Dark gray
    background: "#000000", // Black
  },
];

interface GeometricPatternProps {
  seed: string | number;
  complexity?: number;
  rotationSpeed?: number;
  blackHoleCount?: number;
  theme?: ColorTheme;
}

// Enhanced Random class with more deterministic features
class Random {
  private _seed: number;

  constructor(seedInput: string | number) {
    if (typeof seedInput === "number") {
      this._seed = Math.abs(seedInput);
    } else {
      const seedStr = String(seedInput);
      this._seed = Math.abs(
        seedStr.split("").reduce((acc, char) => {
          const hash = (acc << 5) - acc + char.charCodeAt(0);
          return hash & hash;
        }, 0),
      );
    }
  }

  get seed(): number {
    return this._seed;
  }

  next(): number {
    this._seed = (this._seed * 16807) % 2147483647;
    return (this._seed - 1) / 2147483646;
  }

  range(min: number, max: number): number {
    return min + this.next() * (max - min);
  }

  // Pick random item from array
  pick<T>(array: T[]): T {
    return array[Math.floor(this.range(0, array.length))];
  }

  // Get a value based on character frequency in the seed
  getCharacteristicValue(seedStr: string, min: number, max: number): number {
    const str = String(seedStr).toLowerCase();
    const vowels = (str.match(/[aeiou]/g) || []).length / str.length;
    const consonants =
      (str.match(/[bcdfghjklmnpqrstvwxyz]/g) || []).length / str.length;
    const numbers = (str.match(/[0-9]/g) || []).length / str.length;
    const spaces = (str.match(/\s/g) || []).length / str.length;

    // Use these characteristics to generate a value
    const value =
      vowels * 0.3 + consonants * 0.4 + numbers * 0.2 + spaces * 0.1;
    return min + value * (max - min);
  }

  // Get theme based on seed characteristics
  getTheme(seedStr: string): ColorTheme {
    const str = String(seedStr).toLowerCase();

    // Calculate theme index based on various characteristics
    const length = str.length;
    const firstChar = str.charCodeAt(0);
    const lastChar = str.charCodeAt(str.length - 1);
    const sum = str
      .split("")
      .reduce((acc, char) => acc + char.charCodeAt(0), 0);

    const index =
      Math.abs(sum + firstChar + lastChar + length) % colorThemes.length;
    return colorThemes[index];
  }

  // Get pattern based on seed characteristics
  getPattern(seedStr: string): PatternType {
    const str = String(seedStr).toLowerCase();

    // Use more characteristics for pattern selection
    const wordCount = str.split(/\s+/).length;
    const avgWordLength = str.replace(/\s+/g, "").length / wordCount;
    const specialChars = (str.match(/[^a-zA-Z0-9\s]/g) || []).length;
    const vowelRatio = (str.match(/[aeiou]/g) || []).length / str.length;
    const consonantRatio =
      (str.match(/[bcdfghjklmnpqrstvwxyz]/g) || []).length / str.length;
    const numberRatio = (str.match(/[0-9]/g) || []).length / str.length;

    // Create weighted categories based on characteristics
    const categories = {
      philosophical: vowelRatio * 2 + specialChars / str.length,
      mathematical: numberRatio * 2 + consonantRatio,
      natural: avgWordLength / 5 + vowelRatio,
      technical: consonantRatio + numberRatio,
      meditative: (str.length % 5) / 5 + vowelRatio,
    };

    // Select category based on highest weight
    const category = Object.entries(categories).reduce((a, b) =>
      a[1] > b[1] ? a : b,
    )[0];

    // Map categories to pattern groups
    const patternGroups = {
      philosophical: [
        "wisdom",
        "dialectic",
        "paradox",
        "synthesis",
        "causality",
      ],
      mathematical: ["fibonacci", "fractal", "binary", "quantum", "matrix"],
      natural: ["spiral", "wave", "helix", "crystal", "nebula"],
      technical: ["circuit", "recursion", "emergence", "entropy", "lattice"],
      meditative: ["breath", "meditation", "harmony", "balance", "mindflow"],
    };

    // Use seed to deterministically select from the category
    const selectedGroup = patternGroups[category as keyof typeof patternGroups];
    const patternIndex =
      Math.abs(
        str.split("").reduce((acc, char) => {
          const hash = (acc << 5) - acc + char.charCodeAt(0);
          return hash & hash;
        }, 0),
      ) % selectedGroup.length;

    return selectedGroup[patternIndex] as PatternType;
  }
}

// Mathematical functions for particle behavior
const calculateForce = (distance: number, charge: number): number => {
  const minDistance = 0.1;
  const safeDist = Math.max(distance, minDistance);
  return (charge / (safeDist * safeDist)) * 0.1;
};

// Add glitch effect utilities after the Random class
interface GlitchEffect {
  active: boolean;
  intensity: number;
  duration: number;
  startTime: number;
  type: "displacement" | "pixelation" | "noise" | "slice";
}

class GlitchManager {
  private effects: GlitchEffect[] = [];
  private random: Random;
  private lastGlitchTime: number = 0;
  private minInterval: number = 2.5; // Even more contemplative pacing
  private maxInterval: number = 8;
  private baseIntensity: number = 0.6; // Even subtler effects

  constructor(random: Random) {
    this.random = random;
  }

  update(time: number) {
    // Clean up expired effects
    this.effects = this.effects.filter(
      (effect) => time - effect.startTime < effect.duration,
    );

    // Add new effects randomly
    if (
      time - this.lastGlitchTime >
      this.random.range(this.minInterval, this.maxInterval)
    ) {
      // Add multiple effects at once sometimes
      const numEffects = Math.random() < 0.3 ? 2 : 1;
      for (let i = 0; i < numEffects; i++) {
        this.addRandomEffect(time);
      }
      this.lastGlitchTime = time;
    }
  }

  addRandomEffect(time: number) {
    const types: GlitchEffect["type"][] = [
      "displacement",
      "pixelation",
      "noise",
      "slice",
    ];
    const effect: GlitchEffect = {
      active: true,
      intensity: this.random.range(0.4, 1.0) * this.baseIntensity, // Increased minimum intensity
      duration: this.random.range(0.2, 0.8), // Increased duration range
      startTime: time,
      type: this.random.pick(types),
    };
    this.effects.push(effect);
  }

  applyGlitchEffects(position: Vector3, time: number) {
    this.effects.forEach((effect) => {
      const progress = (time - effect.startTime) / effect.duration;
      const strength = effect.intensity * Math.sin(progress * Math.PI);

      switch (effect.type) {
        case "displacement":
          position.x += Math.sin(time * 80) * strength * 0.8; // Increased frequency and amplitude
          position.y += Math.cos(time * 60) * strength * 0.8;
          break;
        case "pixelation":
          const grid = 0.3 * strength; // Reduced grid size for more extreme pixelation
          position.x = Math.round(position.x / grid) * grid;
          position.y = Math.round(position.y / grid) * grid;
          position.z = Math.round(position.z / grid) * grid;
          break;
        case "noise":
          position.add(
            new Vector3(
              this.random.range(-1, 1) * strength * 0.5, // Increased noise amplitude
              this.random.range(-1, 1) * strength * 0.5,
              this.random.range(-1, 1) * strength * 0.5,
            ),
          );
          break;
        case "slice":
          if (Math.sin(time * 30) > 0) {
            // Increased slice frequency
            position.x += Math.sin(position.y * 8) * strength * 1.2; // Increased slice intensity
          } else {
            position.y += Math.sin(position.x * 8) * strength * 1.2;
          }
          break;
      }
    });
  }

  getActiveEffects(): GlitchEffect[] {
    return this.effects;
  }
}

const GeometricLayer: React.FC<{
  radius: number;
  elements: number;
  random: Random;
  rotationOffset: number;
  baseGeometry: GeometryType;
  layerIndex: number;
  rotationSpeed: number;
  blackHoles: BlackHole[]; // Add black holes parameter
  color: string;
}> = ({
  radius,
  elements,
  random,
  rotationOffset,
  baseGeometry,
  layerIndex,
  rotationSpeed,
  blackHoles,
  color,
}) => {
  const groupRef = useRef<THREE.Group>(null);
  const innerGroupRef = useRef<THREE.Group>(null);
  const timeRef = useRef<number>(0);
  const particlesRef = useRef<
    Array<{
      position: Vector3;
      velocity: Vector3;
      charge: number;
      phase: number;
      frequency: number;
      amplitude: number;
      traceOffset: number;
      traceSpeed: number;
    }>
  >([]);
  const glitchManager = useMemo(() => new GlitchManager(random), [random]);

  // Initialize particles with more varied properties
  useMemo(() => {
    particlesRef.current = Array.from({ length: elements }).map((_, i) => {
      const normalizedI = i / elements;
      const phase = random.range(0, Math.PI * 2);
      const charge = random.range(-1, 1);
      const frequency = random.range(0.5, 2);
      const amplitude = random.range(0.3, 1);

      return {
        position: new Vector3(),
        velocity: new Vector3(
          random.range(-0.02, 0.02),
          random.range(-0.02, 0.02),
          random.range(-0.02, 0.02),
        ),
        charge,
        phase,
        frequency,
        amplitude,
        traceOffset: random.range(0, Math.PI * 2), // For line tracing effect
        traceSpeed: random.range(0.3, 1.2), // Speed of tracing
      };
    });
  }, [elements, random]);

  const pattern = random.getPattern(String(random.seed));
  const timeScale = random.range(0.5, 2);
  const interactionStrength = random.range(0.1, 0.3);

  // Function to calculate black hole influence on a position
  const applyBlackHoleEffect = (position: Vector3, time: number) => {
    blackHoles.forEach((blackHole) => {
      const distanceToHole = position.clone().sub(blackHole.position);
      const distance = distanceToHole.length();

      if (distance < blackHole.radius) {
        // Enhanced pulsing effect
        const timeEffect =
          (Math.sin(time * blackHole.style.pulseSpeed + blackHole.phase) * 0.5 +
            Math.sin(
              time * blackHole.style.pulseSpeed * 1.5 + blackHole.phase,
            ) *
              0.3 +
            Math.sin(
              time * blackHole.style.pulseSpeed * 2.3 + blackHole.phase,
            ) *
              0.2) *
            blackHole.style.pulseIntensity +
          (1 - blackHole.style.pulseIntensity);

        const pullStrength =
          (1 - distance / blackHole.radius) * blackHole.strength * timeEffect;

        if (
          blackHole.connectedTo &&
          timeEffect > blackHole.style.teleportThreshold
        ) {
          // Enhanced exit position with spiral motion
          const exitAngle = time * 2 + blackHole.phase;
          const exitRadius =
            blackHole.connectedTo.radius * blackHole.style.exitSpread;
          const exitOffset = new Vector3(
            Math.cos(exitAngle) * exitRadius,
            Math.sin(time * 3) * exitRadius * 0.5,
            Math.sin(exitAngle) * exitRadius,
          );

          position.copy(blackHole.connectedTo.position.clone().add(exitOffset));
        } else {
          // Enhanced pull effect with spiral motion
          const pullVector = distanceToHole
            .normalize()
            .multiplyScalar(pullStrength);
          position.sub(pullVector);

          // Add more complex spiral effect
          const spiralAngle =
            time * 3 + (distance / blackHole.radius) * Math.PI;
          const spiral = new Vector3(
            Math.cos(spiralAngle) * pullStrength,
            Math.sin(spiralAngle) * pullStrength,
            Math.cos(spiralAngle * 1.5) * pullStrength,
          ).multiplyScalar(0.3 * timeEffect);

          position.add(spiral);
        }
      }
    });
  };

  useFrame((state: RootState) => {
    const time = state.clock.getElapsedTime() * timeScale;
    timeRef.current = time;

    // Update glitch effects
    glitchManager.update(time);

    // Update particle positions with enhanced patterns and glitch effects
    particlesRef.current.forEach((particle, i) => {
      const others = particlesRef.current.filter((_, j) => j !== i);
      const forces = new Vector3();

      others.forEach((other) => {
        const direction = other.position.clone().sub(particle.position);
        const distance = direction.length();
        const force = calculateForce(distance, particle.charge * other.charge);
        direction.normalize().multiplyScalar(force);
        forces.add(direction);
      });

      switch (pattern) {
        case "spiral":
          const spiralAngle =
            time * particle.frequency + (i / elements) * Math.PI * 10;
          const spiralRadius = radius * (0.2 + (i / elements) * 0.8);
          particle.position.set(
            Math.cos(spiralAngle) * spiralRadius,
            Math.sin(spiralAngle) * spiralRadius,
            Math.cos(time * particle.frequency + particle.phase) *
              (radius * 0.3),
          );
          break;

        case "wave":
          const waveX = (i / elements) * radius * 2 - radius;
          const waveY =
            Math.sin(time * particle.frequency + (i / elements) * Math.PI * 4) *
            radius *
            0.3;
          const waveZ =
            Math.cos(time * particle.frequency * 0.5 + particle.phase) *
            radius *
            0.2;
          particle.position.set(waveX, waveY, waveZ);
          break;

        case "helix":
          const helixAngle =
            (i / elements) * Math.PI * 4 + time * particle.frequency;
          const helixRadius = radius * 0.8;
          const helixHeight = ((i / elements) * 2 - 1) * radius;
          particle.position.set(
            Math.cos(helixAngle) * helixRadius,
            helixHeight + Math.sin(time * 0.5) * radius * 0.2,
            Math.sin(helixAngle) * helixRadius,
          );
          break;

        case "fibonacci":
          const goldenRatio = (1 + Math.sqrt(5)) / 2;
          const angle = i * goldenRatio * Math.PI * 2;
          const r = Math.sqrt(i / elements) * radius;
          particle.position.set(
            Math.cos(angle + time * 0.1) * r,
            Math.sin(angle + time * 0.1) * r,
            Math.cos(time * 0.2 + particle.phase) * (radius * 0.2),
          );
          break;

        case "vortex":
          const vortexAngle = time * 0.2 + (i / elements) * Math.PI * 2;
          const vortexRadius =
            radius * (0.5 + Math.sin(time * 0.3 + particle.phase) * 0.2);
          particle.position.set(
            Math.cos(vortexAngle) * vortexRadius,
            Math.sin(vortexAngle) * vortexRadius,
            Math.sin(time + particle.phase) * (radius * 0.3),
          );
          break;

        case "lattice":
          const latticeSize = Math.ceil(Math.sqrt(elements));
          const x =
            ((i % latticeSize) - latticeSize / 2) * (radius / latticeSize) * 2;
          const y =
            (Math.floor(i / latticeSize) - latticeSize / 2) *
            (radius / latticeSize) *
            2;
          const z = Math.sin(time + particle.phase) * (radius * 0.2);
          particle.position.set(x, y, z);
          break;

        case "quantum":
          const quantumRadius =
            radius * (0.8 + Math.sin(time * 0.5 + particle.phase) * 0.2);
          const quantumAngle = i * ((Math.PI * 2) / elements) + time * 0.1;
          particle.position.set(
            Math.cos(quantumAngle) * quantumRadius * Math.cos(time * 0.3),
            Math.sin(quantumAngle) * quantumRadius * Math.sin(time * 0.3),
            Math.cos(time + particle.phase) * (radius * 0.4),
          );
          break;

        case "chaos":
          particle.velocity.add(forces.multiplyScalar(interactionStrength));
          particle.velocity.multiplyScalar(0.99); // Damping
          particle.position.add(particle.velocity);

          // Contain particles within bounds
          if (particle.position.length() > radius * 1.5) {
            particle.position.normalize().multiplyScalar(radius * 1.5);
            particle.velocity.multiplyScalar(-0.5);
          }
          break;

        case "mandala":
          const mandalaAngle = (i / elements) * Math.PI * 2 + time * 0.1;
          const mandalaRadius = radius * Math.sin((i / elements) * Math.PI * 4);
          particle.position.set(
            Math.cos(mandalaAngle) * mandalaRadius,
            Math.sin(mandalaAngle) * mandalaRadius,
            Math.cos(time + particle.phase) * (radius * 0.2),
          );
          break;

        case "starburst":
          const burstAngle = (i / elements) * Math.PI * 2;
          const burstRadius =
            radius *
            (0.2 + Math.pow(Math.sin(time + (i / elements) * Math.PI), 2));
          particle.position.set(
            Math.cos(burstAngle) * burstRadius,
            Math.sin(burstAngle) * burstRadius,
            Math.sin(time * 0.5 + particle.phase) * (radius * 0.3),
          );
          break;

        case "nebula":
          const nebulaT = time * 0.2 + (i / elements) * Math.PI * 2;
          const nebulaR = radius * (0.5 + Math.sin(nebulaT) * 0.3);
          particle.position.set(
            Math.cos(nebulaT) * nebulaR * Math.sin(time * 0.3),
            Math.sin(nebulaT) * nebulaR * Math.cos(time * 0.3),
            Math.sin(time * 0.4 + particle.phase) * (radius * 0.4),
          );
          break;

        case "crystal":
          const crystalAngle = Math.floor(i / 3) * (Math.PI / 6) + time * 0.1;
          const crystalLayer = i % 3;
          const crystalRadius = radius * (0.3 + crystalLayer * 0.2);
          particle.position.set(
            Math.cos(crystalAngle) * crystalRadius,
            Math.sin(crystalAngle) * crystalRadius,
            (crystalLayer - 1) * radius * 0.3,
          );
          break;

        case "dna":
          const dnaT = (i / elements) * Math.PI * 10 + time;
          const dnaRadius = radius * 0.5;
          const dnaHeight = ((i / elements) * 2 - 1) * radius;
          particle.position.set(
            Math.cos(dnaT) * dnaRadius,
            dnaHeight,
            Math.sin(dnaT) * dnaRadius,
          );
          break;

        case "circuit":
          const circuitGridSize = Math.ceil(Math.sqrt(elements));
          const circuitX =
            ((i % circuitGridSize) - circuitGridSize / 2) *
            (radius / circuitGridSize) *
            2;
          const circuitY =
            (Math.floor(i / circuitGridSize) - circuitGridSize / 2) *
            (radius / circuitGridSize) *
            2;
          const circuitZ =
            Math.sin(time + (circuitX * circuitY) / (radius * radius)) *
            (radius * 0.2);
          particle.position.set(circuitX, circuitY, circuitZ);
          break;

        case "origami":
          const foldAngle = (i / elements) * Math.PI * 2;
          const foldRadius = radius * Math.abs(Math.sin(time + foldAngle));
          const foldHeight = Math.cos(foldAngle * 2) * radius * 0.3;
          particle.position.set(
            Math.cos(foldAngle) * foldRadius,
            Math.sin(foldAngle) * foldRadius,
            foldHeight,
          );
          break;

        case "constellation":
          const starAngle = (i / elements) * Math.PI * 2;
          const starRadius = radius * (0.5 + Math.random() * 0.5);
          const starPhase = Math.floor(i / 3) * (Math.PI / 6);
          particle.position.set(
            Math.cos(starAngle + starPhase) * starRadius,
            Math.sin(starAngle + starPhase) * starRadius,
            Math.sin(time * 0.3 + particle.phase) * (radius * 0.2),
          );
          break;

        case "matrix":
          const matrixColumns = Math.ceil(Math.sqrt(elements));
          const columnOffset = (i % matrixColumns) * (radius / matrixColumns);
          const dropSpeed = particle.frequency * 0.5;
          const dropPhase = particle.phase * 10;
          const matrixX = columnOffset - radius / 2;
          const matrixY =
            ((time * dropSpeed + dropPhase) % (radius * 2)) - radius;
          const matrixZ =
            Math.sin(time * 0.2 + particle.phase) * (radius * 0.1);
          particle.position.set(matrixX, matrixY, matrixZ);
          break;

        case "pulse":
          const pulseAngle = (i / elements) * Math.PI * 2;
          const pulsePhase =
            time * particle.frequency + (i / elements) * Math.PI;
          const pulseRadius = radius * (0.5 + Math.sin(pulsePhase) * 0.3);
          const pulseHeight = Math.cos(pulsePhase * 2) * radius * 0.2;
          particle.position.set(
            Math.cos(pulseAngle) * pulseRadius,
            pulseHeight,
            Math.sin(pulseAngle) * pulseRadius,
          );
          break;

        case "ripple":
          const rippleAngle = (i / elements) * Math.PI * 2;
          const rippleDistance = (i / elements) * radius;
          const rippleWave =
            Math.sin(time * 2 - rippleDistance * 0.5) * radius * 0.2;
          particle.position.set(
            Math.cos(rippleAngle) * rippleDistance,
            rippleWave,
            Math.sin(rippleAngle) * rippleDistance,
          );
          break;

        case "fractal":
          const iteration = Math.floor(i / (elements / 4));
          const subIndex = i % (elements / 4);
          const fractalAngle = (subIndex / (elements / 4)) * Math.PI * 2;
          const fractalRadius = radius * Math.pow(0.5, iteration);
          const fractalOffset = radius * (1 - Math.pow(0.5, iteration - 1));
          const fractalSpin = time * (1 + iteration * 0.5);
          particle.position.set(
            Math.cos(fractalAngle + fractalSpin) * fractalRadius +
              fractalOffset,
            Math.sin(fractalAngle + fractalSpin) * fractalRadius,
            (iteration - 1.5) * radius * 0.2,
          );
          break;

        case "infinity":
          const infinityT = time * 0.2;
          const infinityScale = 0.8 + Math.sin(time * 0.1) * 0.2;
          const infinityX = Math.sin(infinityT) * radius * infinityScale;
          const infinityY =
            Math.sin(infinityT * 2) * radius * 0.5 * infinityScale;
          const infinityZ =
            Math.cos(infinityT + particle.phase) * (radius * 0.2);
          particle.position.set(infinityX, infinityY, infinityZ);
          break;

        case "balance":
          const balanceAngle = (i / elements) * Math.PI * 2;
          const balanceRadius =
            radius * (0.3 + Math.sin(time * 0.2 + balanceAngle) * 0.2);
          const balanceOffset =
            Math.sin(balanceAngle * 2 + time * 0.3) * radius * 0.3;
          particle.position.set(
            Math.cos(balanceAngle) * balanceRadius,
            Math.sin(balanceAngle) * balanceRadius + balanceOffset,
            Math.sin(time * 0.2 + particle.phase) * (radius * 0.2),
          );
          break;

        case "mindflow":
          const flowTime = time * 0.3;
          const flowPhase = (i / elements) * Math.PI * 4;
          const flowRadius =
            radius * (0.6 + Math.sin(flowTime + flowPhase) * 0.2);
          const flowHeightOffset =
            Math.sin(flowTime * 2 + flowPhase) * radius * 0.3;
          particle.position.set(
            Math.cos(flowPhase + flowTime) * flowRadius,
            flowHeightOffset,
            Math.sin(flowPhase + flowTime) * flowRadius,
          );
          break;

        case "recursion":
          const depth = Math.floor(i / (elements / 4));
          const subPos = i % (elements / 4);
          const recursionAngle = (subPos / (elements / 4)) * Math.PI * 2;
          const recursionRadius = radius * Math.pow(0.7, depth);
          const recursionOffset = new Vector3(
            Math.cos(recursionAngle + time * (1 + depth * 0.2)) *
              recursionRadius,
            (depth - 1.5) * radius * 0.3,
            Math.sin(recursionAngle + time * (1 + depth * 0.2)) *
              recursionRadius,
          );
          particle.position.copy(recursionOffset);
          break;

        case "binary":
          const level = Math.floor(Math.log2(i + 1));
          const position = i + 1 - Math.pow(2, level);
          const spacing = radius / (level + 1);
          const xPos = (position - Math.pow(2, level) / 2) * spacing;
          const yPos = -level * spacing * 0.8;
          const zOffset = Math.sin(time + particle.phase) * (radius * 0.1);
          particle.position.set(xPos, yPos, zOffset);
          break;

        case "meditation":
          const meditationPhase = time * 0.15; // Slower movement for meditation
          const breathCycle = Math.sin(meditationPhase) * 0.5 + 0.5;
          const meditationRadius = radius * (0.6 + breathCycle * 0.2);
          const meditationAngle = (i / elements) * Math.PI * 2;
          particle.position.set(
            Math.cos(meditationAngle) *
              meditationRadius *
              (1 + Math.sin(meditationPhase * 0.5) * 0.1),
            Math.sin(time * 0.1) * radius * 0.2 * breathCycle,
            Math.sin(meditationAngle) *
              meditationRadius *
              (1 + Math.cos(meditationPhase * 0.5) * 0.1),
          );
          break;

        case "breath":
          const breathPhase = time * 0.2;
          // Box breathing pattern (4-4-4-4)
          const boxBreathDuration = 4; // Each phase is 4 seconds
          const cycleTime = breathPhase % (boxBreathDuration * 4);
          const cyclePhase = Math.floor(cycleTime / boxBreathDuration);
          let breathProgress;

          // Use seed to determine slight variations in timing
          const breathVariation = random.range(0.9, 1.1);
          const adjustedDuration = boxBreathDuration * breathVariation;

          switch (cyclePhase) {
            case 0: // Inhale
              breathProgress =
                (cycleTime % boxBreathDuration) / boxBreathDuration;
              break;
            case 1: // Hold
              breathProgress = 1;
              break;
            case 2: // Exhale
              breathProgress =
                1 - (cycleTime % boxBreathDuration) / boxBreathDuration;
              break;
            default: // Rest
              breathProgress = 0;
          }

          const breathAngle = (i / elements) * Math.PI * 2;
          const breathRadius = radius * (0.4 + breathProgress * 0.4);
          particle.position.set(
            Math.cos(breathAngle) *
              breathRadius *
              (1 + Math.sin(breathPhase * 0.3) * 0.1),
            (breathProgress - 0.5) * radius * 0.4,
            Math.sin(breathAngle) *
              breathRadius *
              (1 + Math.cos(breathPhase * 0.3) * 0.1),
          );
          break;

        case "harmony":
          const harmonyBase = time * 0.2;
          const harmonyPhase = (i / elements) * Math.PI * 2;
          const harmonyWave1 = Math.sin(harmonyBase + harmonyPhase);
          const harmonyWave2 = Math.cos(harmonyBase * 1.618 + harmonyPhase);
          const harmonyRadius =
            radius * (0.6 + harmonyWave1 * harmonyWave2 * 0.2);
          particle.position.set(
            Math.cos(harmonyPhase) * harmonyRadius * (1 + harmonyWave1 * 0.2),
            Math.sin(harmonyBase * 0.5) * radius * 0.3,
            Math.sin(harmonyPhase) * harmonyRadius * (1 + harmonyWave2 * 0.2),
          );
          break;

        case "paradox":
          const paradoxTime = time * 0.25;
          const paradoxAngle = (i / elements) * Math.PI * 2;
          const innerSpiral = paradoxAngle + paradoxTime;
          const outerSpiral = -paradoxAngle + paradoxTime * 0.5;
          const paradoxRadius =
            radius *
            (0.3 +
              Math.abs(Math.sin(innerSpiral) * Math.cos(outerSpiral)) * 0.5);
          particle.position.set(
            Math.cos(innerSpiral) * paradoxRadius,
            Math.sin(outerSpiral) * paradoxRadius,
            Math.cos(paradoxTime + particle.phase) * radius * 0.2,
          );
          break;

        case "emergence":
          const emergenceTime = time * 0.3;
          // Create a deterministic cellular automaton-like behavior
          const cellIndex = Math.floor(i / 3);
          const neighborIndices = [
            (cellIndex - 1 + elements) % elements,
            (cellIndex + 1) % elements,
          ];

          // Use seed to generate stable rules
          const ruleSet = new Array(8)
            .fill(0)
            .map((_, i) => Math.round(random.range(0, 1)));

          // Calculate cell state based on neighbors
          const cellState = neighborIndices.reduce((state, idx) => {
            const neighborPhase = (idx / elements) * Math.PI * 2;
            const neighborValue = Math.sin(emergenceTime + neighborPhase);
            return state + (neighborValue > 0 ? 1 : 0);
          }, 0);

          const ruleIndex = cellState % ruleSet.length;
          const emergenceState = ruleSet[ruleIndex];

          const emergenceBase = radius * (0.4 + emergenceState * 0.4);
          const emergenceAngle = (i / elements) * Math.PI * 2 + emergenceTime;
          const emergenceHeight =
            Math.sin(emergenceTime * 0.5 + (i / elements) * Math.PI * 4) *
            radius *
            0.3;

          particle.position.set(
            Math.cos(emergenceAngle) *
              emergenceBase *
              (1 + Math.sin(emergenceTime * 0.7) * 0.2),
            emergenceHeight * emergenceState,
            Math.sin(emergenceAngle) *
              emergenceBase *
              (1 + Math.cos(emergenceTime * 0.7) * 0.2),
          );
          break;

        case "wisdom":
          const wisdomTime = time * 0.25;
          const wisdomSeed = random.range(0, 1000);
          const growthRate = random.range(0.3, 0.7);
          const branchAngle = (i / elements) * Math.PI * 2;
          const growthPhase =
            Math.sin(wisdomTime * growthRate + wisdomSeed) * 0.5 + 0.5;
          const branchLength = radius * (0.3 + growthPhase * 0.5);
          const wisdomHeightOffset = radius * growthPhase * 0.4;

          particle.position.set(
            Math.cos(branchAngle) *
              branchLength *
              (1 + Math.sin(wisdomTime + wisdomSeed) * 0.2),
            wisdomHeightOffset + Math.sin(wisdomTime * 0.5) * radius * 0.2,
            Math.sin(branchAngle) *
              branchLength *
              (1 + Math.cos(wisdomTime + wisdomSeed) * 0.2),
          );
          break;

        case "dialectic":
          const dialecticTime = time * 0.3;
          const cyclePhaseD = (i / elements) * Math.PI * 2;
          const thesis = Math.sin(dialecticTime + cyclePhaseD);
          const antithesis = Math.cos(dialecticTime + cyclePhaseD);
          const synthesis = (thesis + antithesis) * 0.5;

          const dialecticRadius = radius * (0.4 + Math.abs(synthesis) * 0.4);
          const dialecticAngle =
            cyclePhaseD + dialecticTime * (1 + synthesis * 0.5);

          particle.position.set(
            Math.cos(dialecticAngle) * dialecticRadius * (1 + thesis * 0.2),
            synthesis * radius * 0.4,
            Math.sin(dialecticAngle) * dialecticRadius * (1 + antithesis * 0.2),
          );
          break;

        case "entropy":
          const entropyTime = time * 0.2;
          const orderPhase = Math.max(0, 1 - i / elements);
          const chaosPhase = 1 - orderPhase;
          const entropyAngle = (i / elements) * Math.PI * 2;

          // Deterministic chaos based on seed
          const chaosOffset = new Vector3(
            random.range(-1, 1) * chaosPhase,
            random.range(-1, 1) * chaosPhase,
            random.range(-1, 1) * chaosPhase,
          ).multiplyScalar(radius * 0.3);

          const orderedPosition = new Vector3(
            Math.cos(entropyAngle) * radius * 0.7,
            Math.sin(entropyTime * 0.5) * radius * 0.2,
            Math.sin(entropyAngle) * radius * 0.7,
          );

          particle.position.copy(orderedPosition.add(chaosOffset));
          break;

        case "causality":
          const causalityTime = time * 0.4;
          const causePhase = (i / elements) * Math.PI * 2;
          const effectDelay = random.range(0.1, 0.5);
          const cause = Math.sin(causalityTime + causePhase);
          const effect = Math.sin(
            causalityTime * (1 - effectDelay) + causePhase,
          );

          const causalityRadius = radius * (0.5 + (cause + effect) * 0.2);
          const causalityAngle = causePhase + causalityTime;

          particle.position.set(
            Math.cos(causalityAngle) * causalityRadius,
            (cause * 0.5 + effect * 0.5) * radius * 0.3,
            Math.sin(causalityAngle) * causalityRadius,
          );
          break;

        case "synthesis":
          const synthesisTime = time * 0.35;
          const idea1 = Math.sin(synthesisTime + (i / elements) * Math.PI * 2);
          const idea2 = Math.cos(
            synthesisTime * 1.5 + (i / elements) * Math.PI * 2,
          );
          const combinedIdea = (idea1 + idea2) * 0.5;

          const synthesisRadius = radius * (0.4 + Math.abs(combinedIdea) * 0.4);
          const synthesisAngle = (i / elements) * Math.PI * 2 + synthesisTime;

          particle.position.set(
            Math.cos(synthesisAngle) * synthesisRadius * (1 + idea1 * 0.2),
            combinedIdea * radius * 0.4,
            Math.sin(synthesisAngle) * synthesisRadius * (1 + idea2 * 0.2),
          );
          break;
      }

      // Apply black hole effects after pattern positioning
      applyBlackHoleEffect(particle.position, time);

      // Apply glitch effects after pattern positioning and black hole effects
      glitchManager.applyGlitchEffects(particle.position, time);
    });

    // Update group rotations
    if (groupRef.current && innerGroupRef.current) {
      groupRef.current.rotation.z = time * 0.1 * rotationSpeed + rotationOffset;
      innerGroupRef.current.rotation.y = time * -0.05 * rotationSpeed;
      groupRef.current.rotation.x = Math.sin(time * 0.15 * rotationSpeed) * 0.2;
    }
  });

  return (
    <group ref={groupRef}>
      <group ref={innerGroupRef}>
        {particlesRef.current.map((particle, i) => {
          const scale = random.range(0.8, 1.5);
          const currentTime = timeRef.current;

          // Add glitch effect to opacity
          const glitchEffects = glitchManager.getActiveEffects();
          const glitchOpacityMultiplier = Math.max(
            0.4,
            1 + Math.sin(currentTime * 40) * 0.3,
          );

          // Enhanced line tracing effect with more dynamic opacity variations
          const baseTraceProgress =
            (Math.sin(
              currentTime * particle.traceSpeed + particle.traceOffset,
            ) +
              1) /
            2;
          const positionFactor =
            Math.sin(particle.position.length() / radius + currentTime * 0.2) *
              0.3 +
            0.7;
          const heightFactor =
            Math.cos(particle.position.y / radius + currentTime * 0.15) * 0.2 +
            0.8;
          const phaseFactor =
            Math.sin(particle.phase + currentTime * 0.1) * 0.25 + 0.75;

          const startOpacity = 0.05;
          const maxOpacity = 0.95;
          const opacity =
            (startOpacity +
              (maxOpacity - startOpacity) *
                baseTraceProgress *
                positionFactor *
                heightFactor *
                phaseFactor) *
            glitchOpacityMultiplier;

          return (
            <mesh
              key={i}
              position={[
                particle.position.x,
                particle.position.y,
                particle.position.z,
              ]}
              rotation={[
                currentTime * 0.1 * rotationSpeed + particle.phase,
                currentTime * 0.2 * rotationSpeed + particle.phase,
                currentTime * 0.3 * rotationSpeed + particle.phase,
              ]}
              scale={[scale, scale, scale]}
              visible={false}
            >
              {baseGeometry === "octahedron" && (
                <octahedronGeometry args={[1, 0]} />
              )}
              {baseGeometry === "tetrahedron" && (
                <tetrahedronGeometry args={[1, 0]} />
              )}
              {baseGeometry === "box" && <boxGeometry args={[1, 1, 1]} />}
              {baseGeometry === "icosahedron" && (
                <icosahedronGeometry args={[1, 0]} />
              )}
              {baseGeometry === "dodecahedron" && (
                <dodecahedronGeometry args={[1, 0]} />
              )}
              {baseGeometry === "torus" && (
                <torusGeometry args={[1, 0.3, 3, 6]} />
              )}
              {baseGeometry === "ring" && <ringGeometry args={[0.5, 1, 4]} />}
              {baseGeometry === "plane" && <planeGeometry args={[1, 1]} />}
              {baseGeometry === "cylinder" && (
                <cylinderGeometry args={[0.5, 0.5, 1, 4]} />
              )}
              {baseGeometry === "cone" && <coneGeometry args={[0.5, 1, 4]} />}
              {baseGeometry === "capsule" && (
                <capsuleGeometry args={[0.3, 0.5, 1, 4]} />
              )}
              <meshBasicMaterial visible={false} />
            </mesh>
          );
        })}
        {particlesRef.current.map((particle, i) => {
          const scale = random.range(0.8, 1.5);
          const currentTime = timeRef.current;

          // Add glitch effect to opacity
          const glitchEffects = glitchManager.getActiveEffects();
          const glitchOpacityMultiplier = Math.max(
            0.4,
            1 + Math.sin(currentTime * 40) * 0.3,
          );

          // Enhanced line tracing effect with more dynamic opacity variations
          const baseTraceProgress =
            (Math.sin(
              currentTime * particle.traceSpeed + particle.traceOffset,
            ) +
              1) /
            2;
          const positionFactor =
            Math.sin(particle.position.length() / radius + currentTime * 0.2) *
              0.3 +
            0.7;
          const heightFactor =
            Math.cos(particle.position.y / radius + currentTime * 0.15) * 0.2 +
            0.8;
          const phaseFactor =
            Math.sin(particle.phase + currentTime * 0.1) * 0.25 + 0.75;

          const startOpacity = 0.05;
          const maxOpacity = 0.95;
          const opacity =
            (startOpacity +
              (maxOpacity - startOpacity) *
                baseTraceProgress *
                positionFactor *
                heightFactor *
                phaseFactor) *
            glitchOpacityMultiplier;

          return (
            <lineSegments
              key={`line-${i}`}
              position={[
                particle.position.x,
                particle.position.y,
                particle.position.z,
              ]}
              rotation={[
                currentTime * 0.1 * rotationSpeed + particle.phase,
                currentTime * 0.2 * rotationSpeed + particle.phase,
                currentTime * 0.3 * rotationSpeed + particle.phase,
              ]}
              scale={[scale, scale, scale]}
            >
              <edgesGeometry args={[getGeometry(baseGeometry)]} />
              <lineBasicMaterial
                color={color}
                transparent
                opacity={opacity}
                linewidth={2}
              />
            </lineSegments>
          );
        })}
      </group>
    </group>
  );
};

// Helper function to get geometry for edges
const getGeometry = (type: GeometryType): BufferGeometry => {
  switch (type) {
    case "octahedron":
      return new THREE.OctahedronGeometry(1, 0);
    case "tetrahedron":
      return new THREE.TetrahedronGeometry(1, 0);
    case "box":
      return new THREE.BoxGeometry(1, 1, 1);
    case "icosahedron":
      return new THREE.IcosahedronGeometry(1, 0);
    case "dodecahedron":
      return new THREE.DodecahedronGeometry(1, 0);
    case "torus":
      return new THREE.TorusGeometry(1, 0.3, 3, 6);
    case "ring":
      return new THREE.RingGeometry(0.5, 1, 4);
    case "plane":
      return new THREE.PlaneGeometry(1, 1);
    case "cylinder":
      return new THREE.CylinderGeometry(0.5, 0.5, 1, 4);
    case "cone":
      return new THREE.ConeGeometry(0.5, 1, 4);
    case "capsule":
      return new THREE.CapsuleGeometry(0.3, 0.5, 1, 4);
  }
  // Return a default geometry if none matched
  return new THREE.BoxGeometry(1, 1, 1);
};

const Pattern: React.FC<GeometricPatternProps> = ({
  seed,
  complexity = 50,
  rotationSpeed = 0.5,
  blackHoleCount = 5,
}) => {
  const random = new Random(seed);
  const seedStr = String(seed);

  // Get theme based on seed
  const theme = random.getTheme(seedStr);

  // Adjust complexity based on seed characteristics
  const adjustedComplexity = Math.floor(
    complexity * random.getCharacteristicValue(seedStr, 0.8, 1.2),
  );

  // Adjust rotation speed based on seed characteristics
  const adjustedRotationSpeed =
    rotationSpeed * random.getCharacteristicValue(seedStr, 0.7, 1.3);

  // Adjust black hole count based on seed length and characteristics
  const adjustedBlackHoleCount = Math.max(
    2,
    Math.min(
      7,
      Math.floor(
        blackHoleCount * random.getCharacteristicValue(seedStr, 0.6, 1.4),
      ),
    ),
  );

  const { camera } = useThree();

  // Adjusted scene dimensions for larger complexity
  const layers = Math.floor(adjustedComplexity * 0.5) + 4;
  const maxRadius = (layers + 1) * 3.5;
  const sceneDepth = maxRadius * 2;

  // Adjusted camera settings for slower, more stable movement
  const fov = 50;
  const fovRadians = (fov * Math.PI) / 180;
  const optimalDistance = sceneDepth / 2 / Math.tan(fovRadians / 2);

  // Slower camera movement
  const baseSpeed = random.range(0.005, 0.01) * adjustedRotationSpeed;
  const amplitudeX = random.range(12, 16);
  const amplitudeY = random.range(10, 14);
  const amplitudeZ = random.range(6, 8);

  // Generate black hole pairs with optimized parameters
  const blackHoles = useMemo(() => {
    const holes: BlackHole[] = [];
    const maxRadius = (Math.floor(adjustedComplexity * 0.5) + 4 + 1) * 3.5;

    // Adjusted global style variations for larger scene
    const globalStyle = {
      baseStrength: random.range(0.1, 0.3),
      baseRadius: random.range(4, 8),
      strengthVariation: random.range(0.7, 1.2),
      radiusVariation: random.range(0.9, 1.1),
      phaseSpeed: random.range(0.3, 0.6),
    };

    for (let i = 0; i < adjustedBlackHoleCount; i++) {
      // Optimized style for each pair
      const pairStyle: BlackHoleStyle = {
        pulseSpeed: random.range(0.3, 0.8) * globalStyle.phaseSpeed,
        pulseIntensity: random.range(0.4, 0.6),
        teleportThreshold: random.range(0.75, 0.85),
        exitSpread: random.range(0.4, 0.6),
        colorShift: random.range(0, Math.PI * 2),
      };

      // More strategic positioning for larger scene
      const layerIndex = Math.floor(i * (layers / adjustedBlackHoleCount));
      const baseRadius = maxRadius * (layerIndex / layers);

      const angle1 = random.range(0, Math.PI * 2);
      const angle2 = angle1 + Math.PI + random.range(-0.5, 0.5);
      const radius1 = baseRadius * random.range(0.8, 1.2);
      const radius2 = baseRadius * random.range(0.8, 1.2);
      const height1 = maxRadius * random.range(-0.3, 0.3);
      const height2 = -height1 + maxRadius * random.range(-0.1, 0.1);

      const hole1: BlackHole = {
        position: new Vector3(
          Math.cos(angle1) * radius1,
          height1,
          Math.sin(angle1) * radius1,
        ),
        strength:
          globalStyle.baseStrength *
          globalStyle.strengthVariation *
          random.range(0.9, 1.1),
        radius:
          globalStyle.baseRadius *
          globalStyle.radiusVariation *
          random.range(0.9, 1.1),
        phase: random.range(0, Math.PI * 2),
        style: pairStyle,
      };

      const hole2: BlackHole = {
        position: new Vector3(
          Math.cos(angle2) * radius2,
          height2,
          Math.sin(angle2) * radius2,
        ),
        strength: hole1.strength * random.range(0.95, 1.05),
        radius: hole1.radius * random.range(0.95, 1.05),
        phase: hole1.phase + Math.PI + random.range(-0.2, 0.2),
        style: pairStyle,
      };

      hole1.connectedTo = hole2;
      hole2.connectedTo = hole1;

      holes.push(hole1, hole2);
    }

    return holes;
  }, [seed, adjustedComplexity, adjustedBlackHoleCount]);

  useFrame((state: RootState) => {
    const time = state.clock.getElapsedTime() * adjustedRotationSpeed * 0.5; // Even slower overall movement

    // More gentle and predictable camera movement
    const angle = time * baseSpeed * 0.6;
    const height = Math.sin(time * 0.12) * amplitudeY * 0.5;
    const radius = amplitudeX + Math.sin(time * 0.06) * amplitudeZ * 0.6;
    const verticalAngle = Math.sin(time * 0.09) * 0.2;

    // Start much closer on X axis while maintaining Z distance for depth
    camera.position.x = 0;
    camera.position.y = 0.5;
    camera.position.z =
      Math.sin(angle) * radius * Math.cos(verticalAngle) * 0.25;

    // Smoother camera rotation with gentler look-at target
    camera.up.set(0, 1, 0);
    camera.lookAt(
      Math.sin(time * 0.04) * radius * 0.02,
      Math.cos(time * 0.06) * radius * 0.02,
      0,
    );
  });

  const geometries: GeometryType[] = [
    "octahedron",
    "tetrahedron",
    "box",
    "icosahedron",
    "dodecahedron",
    "torus",
    "ring",
    "plane",
    "cylinder",
    "cone",
    "capsule",
  ];

  return (
    <>
      <ambientLight intensity={0.6} />
      <pointLight
        position={[15, 15, 30]}
        intensity={0.8}
        color={theme.accent}
      />
      <pointLight
        position={[-15, -15, 30]}
        intensity={0.8}
        color={theme.secondary}
      />

      {Array.from({ length: layers }).map((_, i) => {
        const radius = (i + 1) * 3.5;
        const elements = Math.floor(random.range(8, 24)) * 2;
        const rotationOffset = random.range(0, Math.PI * 2);
        const baseGeometry = random.pick(geometries);

        // Vary color based on layer depth
        const layerProgress = i / layers;
        const color =
          layerProgress < 0.3
            ? theme.primary
            : layerProgress < 0.7
              ? theme.secondary
              : theme.accent;

        return (
          <GeometricLayer
            key={i}
            radius={radius}
            elements={elements}
            random={random}
            rotationOffset={rotationOffset}
            baseGeometry={baseGeometry}
            layerIndex={i}
            rotationSpeed={adjustedRotationSpeed}
            blackHoles={blackHoles}
            color={color}
          />
        );
      })}
    </>
  );
};

const GeometricScene: React.FC<GeometricPatternProps> = (props) => {
  // Ensure seed is provided
  if (!props.seed) {
    console.warn('No seed provided, using default seed "default"');
    props = { ...props, seed: "default" };
  }

  // Calculate initial camera distance based on complexity
  const layers = Math.floor((props.complexity || 5) * 0.7) + 4;
  const maxRadius = (layers + 1) * 4.5;
  const sceneDepth = maxRadius * 2;
  const fov = 60;
  const fovRadians = (fov * Math.PI) / 180;
  const initialDistance = sceneDepth / 2 / Math.tan(fovRadians / 2);

  return (
    <div
      style={{
        width: "100%",
        height: "100%",
        position: "absolute",
        top: 0,
        left: 0,
      }}
    >
      <Canvas
        camera={{
          position: [0, 0, initialDistance * 0.6], // Use calculated distance with adjustment
          fov: fov,
          near: 0.1,
          far: initialDistance * 4, // Adjust far plane based on scene size
        }}
      >
        <Pattern {...props} />
      </Canvas>
    </div>
  );
};

export default GeometricScene;
