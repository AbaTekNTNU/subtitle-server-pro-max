import { type ClassValue, clsx } from "clsx";
import { cubicIn, cubicInOut, cubicOut } from "svelte/easing";
import { twMerge } from "tailwind-merge";
import { Vector3 } from "three";
import type { AnimationType } from "./bindings/AnimationType";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function textToVector(text: string): Vector3 | null {
  if (text === "null" || text === "") {
    return null;
  }
  const [x, y, z] = text.split(" ").map(Number);
  return new Vector3(x, y, z);
}

export function animateConversion(
  animType: AnimationType | null,
): ((t: number) => number) | undefined {
  switch (animType) {
    case "In":
      return cubicIn;

    case "Out":
      return cubicOut;

    case "InOut":
      return cubicInOut;

    default:
      return undefined;
  }
}
