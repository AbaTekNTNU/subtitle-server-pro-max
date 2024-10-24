import { type PerspectiveCamera, Vector3 } from "three";

export const pointCamera = (
  camera: PerspectiveCamera,
  old: Vector3,
  target: Vector3,
) => {
  let id = setInterval(() => {
    camera.lookAt(
      old.x + target.x * 0.1,
      old.y + target.y * 0.1,
      old.z + target.z * 0.1,
    );

    if (
      Math.abs(old.x - target.x) < 0.01 &&
      Math.abs(old.y - target.y) < 0.01 &&
      Math.abs(old.z - target.z) < 0.01
    ) {
      clearInterval(id);
    }
  }, 100);
};
