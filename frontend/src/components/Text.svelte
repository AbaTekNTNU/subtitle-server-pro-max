<script lang="ts">
  import type { LoadSong } from "$lib/bindings/LoadSong";
  import { animateConversion } from "$lib/utils";
  import { T, useTask, useThrelte, useLoader } from "@threlte/core";
  import { Text3DGeometry, Suspense, Grid, Sky } from "@threlte/extras";
  import { onMount } from "svelte";
  import { cubicInOut, cubicOut } from "svelte/easing";
  import { tweened } from "svelte/motion";
  import {
    Mesh,
    type BufferGeometry,
    type Material,
    type NormalBufferAttributes,
    type Object3DEventMap,
  } from "three";
  import { FontLoader, GLTFLoader } from "three/examples/jsm/Addons.js";
  import { DEG2RAD } from "three/src/math/MathUtils.js";
  import { Vector3 } from "three/src/math/Vector3.js";

  const gltf = useLoader(GLTFLoader).load("abakua.glb");

  interface Props {
    base: string;
  }
  const { base }: Props = $props();
  let { camera } = useThrelte();

  const cameraPosition = tweened(
    { x: 0, y: 10, z: 150 },
    {
      duration: 250,
      delay: 0,
      easing: cubicInOut,
    },
  );

  let cowRotate = $state(0);

  const cameraRotation = tweened(
    { x: 0, y: 0, z: 0 },
    {
      duration: 250,
      delay: 0,
      easing: cubicInOut,
    },
  );

  const lookAtAnimation = tweened(
    {
      x: 0,
      y: 1,
      z: 0,
    },
    {
      duration: 250,
      delay: 0,
      easing: cubicOut,
    },
  );

  let animateText = $state(
    tweened(
      {
        x: 0,
        y: 0,
        z: 0,
      },
      { duration: 1000, delay: 0, easing: cubicOut },
    ),
  );

  let refs: Mesh<
    BufferGeometry<NormalBufferAttributes>,
    Material | Material[],
    Object3DEventMap
  >[] = $state([]);

  const font = useLoader(FontLoader).load("/font.json");

  let target: Vector3 = $state(new Vector3(0, 1, 0));
  let cam_pos: Vector3 = $state(new Vector3(0, 10, 150));

  useTask((delta) => {
    camera.current.lookAt(
      new Vector3($lookAtAnimation.x, $lookAtAnimation.y, $lookAtAnimation.z),
    );

    camera.current.position.x = $cameraPosition.x;
    camera.current.position.y = $cameraPosition.y;
    camera.current.position.z = $cameraPosition.z;

    camera.current.rotation.x = $cameraRotation.x * DEG2RAD;
    camera.current.rotation.y = $cameraRotation.y * DEG2RAD;
    camera.current.rotation.z = $cameraRotation.z * DEG2RAD;

    if (animate) {
      console.log(refs[active_line ?? 0].position);

      refs[active_line ?? 0].position.x = $animateText.x;
      refs[active_line ?? 0].position.y = $animateText.y;
      refs[active_line ?? 0].position.z = $animateText.z;
    }

    cowRotate += 0.1;
  });

  let animate = $state(false);
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
          lookAtAnimation.set({
            x: song.lines[active_line].cam_look_at.x,
            y: song.lines[active_line].cam_look_at.y,
            z: song.lines[active_line].cam_look_at.z,
          });

          cameraPosition.set({
            x: song.lines[active_line].cam_position.x,
            y: song.lines[active_line].cam_position.y,
            z: song.lines[active_line].cam_position.z,
          });

          cameraRotation.set({
            x: song.lines[active_line].cam_rotation?.x || 0,
            y: song.lines[active_line].cam_rotation?.y || 0,
            z: song.lines[active_line].cam_rotation?.z || 0,
          });

          if (song.lines[active_line].end_position) {
            animateText = tweened(
              {
                x: centerText(
                  song.lines[active_line].position.x,
                  song.lines[active_line].line.length,
                ),
                y: song.lines[active_line].position.y,
                z: song.lines[active_line].position.z,
              },
              {
                duration: 1000,
                delay: 0,
                easing: animateConversion(
                  song.lines[active_line].text_animation,
                ),
              },
            );

            animateText.set({
              x: centerText(
                song.lines[active_line].end_position!.x,
                song.lines[active_line].line.length,
              ),
              y: song.lines[active_line].end_position!.y,
              z: song.lines[active_line].end_position!.z,
            });
            animate = true;
          } else {
            animate = false;
          }

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
    } else if (e.key === "Escape") {
      target.x = -10;
    }
  };

  const centerText = (x: number, length: number): number => {
    return x - (length * 1.6) / 2;
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<T.Group>
  <T.PerspectiveCamera
    makeDefault
    position={[cam_pos.x, cam_pos.y, cam_pos.z]}
    fov={15}
  ></T.PerspectiveCamera>
</T.Group>
<Sky elevation={0.5} />

<T.DirectionalLight position={[10, 10, 10]} castShadow />

<Grid
  cellColor="#ffffff"
  sectionColor="#ffffff"
  sectionThickness={0}
  gridSize={1000}
  fadeDistance={250}
  cellSize={2}
/>

{#if $gltf}
  <T
    is={$gltf.scene}
    scale={1000}
    position={[0, 15, -20]}
    rotation={[20 * DEG2RAD, -10 * DEG2RAD, 0]}
  />
{/if}

{#if song}
  {#each song.lines as line, index}
    <Suspense>
      <T.Mesh
        oncreate={(ref: Mesh) => {
          refs = [...refs, ref];
        }}
        position={[
          centerText(line.position.x, line.line.length),
          line.position.y,
          line.position.z,
        ]}
        rotation={[
          line.rotation ? line.rotation.x * DEG2RAD : 0,
          line.rotation ? line.rotation.y * DEG2RAD : 0,
          line.rotation ? line.rotation.z * DEG2RAD : 0,
        ]}
        scale={0.02}
        visible={index === active_line || visible_lines.includes(index)}
      >
        <Text3DGeometry
          font={$font}
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
