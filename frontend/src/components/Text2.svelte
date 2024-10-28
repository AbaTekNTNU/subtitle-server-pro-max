<script lang="ts">
  import { T, useTask, useThrelte } from "@threlte/core";
  import { Text3DGeometry, Suspense } from "@threlte/extras";
  import { onMount } from "svelte";
  import { Vector3 } from "three/src/math/Vector3.js";

  interface Props {
    base: string;
  }
  const { base }: Props = $props();
  let { camera } = useThrelte();

  let lookAt: Vector3 = $state(new Vector3(0, 1, 0));
  let target: Vector3 = $state(new Vector3(0, 1, 0));

  useTask((delta) => {
    if (Math.abs(target.y - lookAt.y) > 0.01) {
      lookAt.y = lookAt.y + (target.y - lookAt.y) * delta * 5;
      camera.current.lookAt(lookAt);
    }
    if (Math.abs(target.x - lookAt.x) > 0.01) {
      lookAt.x = lookAt.x + (target.x - lookAt.x) * delta * 2;
      camera.current.lookAt(lookAt);
    }
    if (Math.abs(target.z - lookAt.z) > 0.01) {
      lookAt.z = lookAt.z + (target.z - lookAt.z) * delta * 2;
      camera.current.lookAt(lookAt);
    }
  });

  let ev: EventSource;

  let text = $state("Hello\nWorld");
  let alt_text = $state("Hello\nWorld");
  let flip = $state(false);

  onMount(() => {
    ev = new EventSource(`${base}/sse`);

    ev.onmessage = (e) => {
      if (flip) {
        text = e.data;
        target.y = 21;
        flip = !flip;
      } else {
        alt_text = e.data;
        target.y = -19;
        flip = !flip;
      }
    };

    return () => {
      ev.close();
    };
  });

  const handleKeydown = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      target.x = 10;
      console.log("target", target);
    } else if (e.key === "Escape") {
      target.x = -10;
    }
  };
</script>

<svelte:window on:keydown={handleKeydown} />
<T.Group>
  <T.PerspectiveCamera
    makeDefault
    position={[0, 10, 100]}
    fov={15}
    oncreate={(cam) => {
      cam.lookAt(lookAt);
    }}
  ></T.PerspectiveCamera>
</T.Group>

<T.DirectionalLight position={[10, 10, 10]} castShadow />

<Suspense>
  <T.Mesh position={[-10, 20, 0]} scale={0.02} visible={false}>
    <Text3DGeometry
      {text}
      bevelEnabled
      bevelSize={1}
      bevelSegments={20}
      curveSegments={12}
    />
    <T.MeshStandardMaterial color="#FD3F00" />
  </T.Mesh>
</Suspense>

<Suspense>
  <T.Mesh position={[-10, -20, 0]} scale={0.02} rotation.x={-0.2}>
    <Text3DGeometry
      text={alt_text}
      bevelEnabled
      bevelSize={1}
      bevelSegments={20}
      curveSegments={12}
    />
    <T.MeshStandardMaterial color="#FD3F00" />
  </T.Mesh>
</Suspense>
