<script lang="ts">
  import type { LoadSong } from "$lib/bindings/LoadSong";
  import { T, useTask, useThrelte } from "@threlte/core";
  import { Text3DGeometry, Suspense, Grid, Sky } from "@threlte/extras";
  import { Vector3 } from "three/src/math/Vector3.js";

  interface Props {
    base: string;
  }
  const { base }: Props = $props();
  let { camera } = useThrelte();

  let lookAt: Vector3 = $state(new Vector3(0, 1, 0));
  let target: Vector3 = $state(new Vector3(0, 1, 0));

  let cam_pos: Vector3 = $state(new Vector3(0, 10, 150));

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

  let song: LoadSong | null = $state(null);
  let ev_load: EventSource;
  let ev_index: EventSource;
  let active_line: number | null = $state(null);
  let prev_line: number | null = $state(null);

  $effect(() => {
    ev_load = new EventSource(`${base}/load`);
    ev_index = new EventSource(`${base}/line`);

    ev_load.onmessage = (e) => {
      console.log("e.data", e.data);
      song = JSON.parse(e.data);
    };

    ev_index.onmessage = (e) => {
      const data: string = e.data;
      if (data !== "NULL") {
        // Line count is 1-indexed

        prev_line = active_line;
        active_line = parseInt(data) - 1;

        if (song) {
          target.x = song.lines[active_line].position.x;
          target.y = song.lines[active_line].position.y;
          target.z = song.lines[active_line].position.z;
        }
      } else {
        active_line = null;
      }
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
    position={[cam_pos.x, cam_pos.y, cam_pos.z]}
    fov={15}
    oncreate={(cam) => {
      cam.lookAt(lookAt);
    }}
  ></T.PerspectiveCamera>
</T.Group>
<Sky elevation={0.5} />

<T.DirectionalLight position={[10, 10, 10]} castShadow />

<Grid
  position.y={-0.001}
  cellColor="#ffffff"
  sectionColor="#ffffff"
  sectionThickness={0}
  gridSize={1000}
  fadeDistance={250}
  cellSize={2}
/>

{#if song}
  {#each song.lines as line, index}
    <Suspense>
      <T.Mesh
        position={[-15 + line.position.x, line.position.y, line.position.z]}
        scale={0.02}
        visible={index === active_line || index === prev_line}
      >
        <Text3DGeometry
          text={line.line}
          bevelEnabled
          bevelSize={1}
          bevelSegments={20}
          curveSegments={12}
        />
        <T.MeshStandardMaterial color="#FD3F00" />
      </T.Mesh>
    </Suspense>
  {/each}
{/if}
