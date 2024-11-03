<script lang="ts">
  import { post } from "$lib/http";
  import { onMount } from "svelte";

  const { name, lines, base }: { name: string; lines: string[]; base: string } =
    $props();

  let key_repeat: string | null = $state(null);
  let error: string | null = $state(null);

  const handleKeyPress = (e: KeyboardEvent) => {
    e.preventDefault();

    switch (e.key) {
      case "j":
      case "ArrowDown":
        console.log("j pressed");
        post(
          `${base}/song/next`,
          JSON.stringify({ skips: key_repeat ? Number(key_repeat) : 1 }),
        );
        key_repeat = null;
        break;

      case "k":
      case "ArrowUp":
        console.log("k pressed");
        post(
          `${base}/song/next`,
          JSON.stringify({ skips: key_repeat ? -Number(key_repeat) : -1 }),
        );

        key_repeat = null;
        break;

      case "Escape":
        key_repeat = null;
        break;

      case "Backspace":
        post(`${base}/reset`);
        key_repeat = null;
        break;

      case "Enter":
      case " ":
        post(`${base}/song/next`, JSON.stringify({ skips: 1 }));
        key_repeat = null;
        break;

      case "g":
        if (key_repeat === null) {
          return;
        } else {
          post(
            `${base}/song/next`,
            JSON.stringify({
              skips: Number(key_repeat) - (active_line ? active_line + 1 : 0),
            }),
          );
          key_repeat = null;
        }

        break;

      default:
        console.log("Key pressed: ", e.key);

        if (e.key.match(/[0-9]/)) {
          if (key_repeat === null) {
            key_repeat = e.key;
          } else {
            key_repeat += e.key;
          }
        }
        break;
    }
  };

  const handleMousePress = (index: number) => {
    fetch(`${base}/song/next`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        skips: index + 1 - (active_line ? active_line + 1 : 0),
      }),
    });
  };

  let active_line: number | null = $state(null);

  let ev: EventSource;

  let div: HTMLDivElement | null = $state(null);

  onMount(() => {
    ev = new EventSource(`${base}/line`);

    ev.onmessage = (e) => {
      console.log("Message received: ", e.data);

      const data: string = e.data;
      if (data !== "NULL") {
        // Line count is 1-indexed
        active_line = parseInt(data) - 1;

        // Scroll to the active line
        if (div) {
          div.scrollIntoView({ behavior: "smooth", block: "center" });
        }
      } else {
        active_line = null;
      }
    };

    ev.onerror = (e) => {
      console.error("Error occurred: ", e);
      error = "Event listener closed";
    };

    return () => {
      ev.close();
    };
  });
</script>

<svelte:window on:keydown={handleKeyPress} on:beforeunload={() => ev.close()} />
<div class="flex w-full flex-col gap-2">
  {#if error}
    <p class="text-center text-red-500">{error}</p>
  {/if}
  <h1 class="my-4 text-center text-3xl font-bold">{name}</h1>
  <div>
    {#each lines as line, index}
      {#if index === active_line}
        <div class="flex justify-center gap-2" bind:this={div}>
          <p>👉</p>
          <p class="text-center text-2xl font-bold">{line}</p>
        </div>
      {:else}
        <div class="flex justify-center gap-2">
          <p>{index + 1}.</p>
          <button
            class="h-6 text-center"
            onclick={() => handleMousePress(index)}>{line}</button
          >
        </div>
      {/if}
    {/each}
  </div>
</div>
