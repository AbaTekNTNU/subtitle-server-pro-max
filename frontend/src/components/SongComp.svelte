<script lang="ts">
  import { post } from "$lib/http";
  import { onMount } from "svelte";

  const { name, lines, base }: { name: string; lines: string[]; base: string } =
    $props();

  let key_repeat: string | null = $state(null);
  let error: string | null = $state(null);

  const handleKeyPress = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      console.log("Enter pressed");
    } else {
      console.log("Key pressed: ", e.key);
    }

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

  let active_line: number | null = $state(null);

  let ev: EventSource;

  onMount(() => {
    ev = new EventSource(`${base}/line`);

    ev.onmessage = (e) => {
      console.log("Message received: ", e.data);

      const data: string = e.data;
      if (data !== "NULL") {
        // Line count is 1-indexed
        active_line = parseInt(data) - 1;
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
  <h1 class="my-4 text-center text-3xl font-bold">{name}</h1>
  <div>
    {#each lines as line, index}
      {#if index === active_line}
        <p class="text-center text-2xl font-bold">{line}</p>
      {:else}
        <p class="h-6 text-center">{line}</p>
      {/if}
    {/each}
  </div>
  {#if error}
    <p class="text-center text-red-500">{error}</p>
  {/if}
</div>
