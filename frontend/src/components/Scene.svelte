<script lang="ts">
  import { T, useTask } from "@threlte/core";
  import { spring } from "svelte/motion";
  import {
    Text,
    Suspense,
    interactivity,
    Text3DGeometry,
  } from "@threlte/extras";
  interactivity();
  const scale = spring(1);
  let rotation = $state(0);
  useTask((delta) => {
    rotation += delta;
  });

  const accel = $derived(rotation * 10);
</script>

<T.PerspectiveCamera
  makeDefault
  position={[0, 10, 10]}
  fov={90}
  oncreate={(camera) => {
    camera.lookAt(0, 1, 0);
  }}
/>

<T.DirectionalLight position={[10, 10, 10]} castShadow />

<T.Mesh
  position={[0, 4, 0]}
  scale={0.02}
  rotation.x={-0.7}
  rotation.y={rotation}
>
  <Text3DGeometry
    text={"Hello\nWorld"}
    bevelEnabled
    bevelSize={1}
    bevelSegments={20}
    curveSegments={12}
  />
  <T.MeshStandardMaterial color="#FD3F00" />
</T.Mesh>

<T.Mesh
  rotation.y={rotation}
  position.y={1}
  scale={$scale}
  onpointerenter={() => scale.set(1.5)}
  onpointerleave={() => scale.set(1)}
  castShadow
>
  <T.BoxGeometry args={[1, 2, 1]} />
  <T.MeshStandardMaterial color="hotpink" />
</T.Mesh>

<Suspense>
  <Text
    text="Hello from threejs"
    scale={10}
    position={[-4, 5, -1]}
    rotation.x={accel}
  />
</Suspense>

<T.Mesh rotation.x={-Math.PI / 2} receiveShadow>
  <T.CircleGeometry args={[4, 40]} />
  <T.MeshStandardMaterial color="white" />
</T.Mesh>
