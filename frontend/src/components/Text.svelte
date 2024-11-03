<script lang="ts">
  import type { LoadSong } from "$lib/bindings/LoadSong";
  import { T, useTask, useThrelte } from "@threlte/core";
  import { Text3DGeometry, Suspense, Grid, Sky } from "@threlte/extras";
  import { onMount } from "svelte";
  import { tweened } from "svelte/motion";
  import { Vector3 } from "three/src/math/Vector3.js";

  interface Props {
    base: string;
  }

  const cameraSpring = tweened(
    { x: 0, y: 10, z: 150 },
    {
      duration: 200,
      delay: 0,
    },
  );

  const { base }: Props = $props();
  let { camera } = useThrelte();

  camera.current.position.x = $cameraSpring.x;
  camera.current.position.y = $cameraSpring.y;
  camera.current.position.z = $cameraSpring.z;

  const lookAtTween = tweened(
    {
      x: 0,
      y: 1,
      z: 0,
    },
    {
      duration: 200,
      delay: 0,
    },
  );

  let target: Vector3 = $state(new Vector3(0, 1, 0));
  let cam_pos: Vector3 = $state(new Vector3(0, 10, 150));

  useTask(() => {
    camera.current.lookAt(
      new Vector3($lookAtTween.x, $lookAtTween.y, $lookAtTween.z),
    );

    camera.current.position.x = $cameraSpring.x;
    camera.current.position.y = $cameraSpring.y;
    camera.current.position.z = $cameraSpring.z;
  });

  let song: LoadSong | null = $state(null);
  let ev_load: EventSource;
  let ev_index: EventSource;
  let active_line: number | null = $state(null);
  let visible_lines: number[] = $state([]);

  onMount(() => {
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
        active_line = parseInt(data) - 1;

        if (song) {
          lookAtTween.set({
            x: song.lines[active_line].cam_look_at.x,
            y: song.lines[active_line].cam_look_at.y,
            z: song.lines[active_line].cam_look_at.z,
          });

          cameraSpring.set({
            x: song.lines[active_line].cam_position.x,
            y: song.lines[active_line].cam_position.y,
            z: song.lines[active_line].cam_position.z,
          });

          visible_lines = [];
          for (let i = 1; i <= song.lines[active_line].keep_n_last; i++) {
            visible_lines.push(active_line - i);
          }
        }
      } else {
        active_line = null;
      }
    };

    return () => {
      ev_load.close();
      ev_index.close();
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

  const centerText = (x: number, length: number): number => {
    return x - (length * 1.6) / 2;
  };
</script>

<svelte:window on:keydown={handleKeydown} />

{#if song}
  <T.Group>
    <T.PerspectiveCamera
      makeDefault
      position={[cam_pos.x, cam_pos.y, cam_pos.z]}
      fov={15}
      oncreate={(cam) => {
        cam.lookAt(new Vector3($lookAtTween.x, $lookAtTween.y, $lookAtTween.z));
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
          position={[
            centerText(line.position.x, line.line.length),
            line.position.y,
            line.position.z,
          ]}
          scale={0.02}
          visible={index === active_line || visible_lines.includes(index)}
        >
          <Text3DGeometry
            font={"/font.json"}
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
  {/if}{/if}
