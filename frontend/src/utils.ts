import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { Vector3 } from "three";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function textToVector(text: string): Vector3 | null {
  if (text === "null" || text === "") {
    return null
  }
  const [x, y, z] = text.split(" ").map(Number);
  return new Vector3(x, y, z);
}
