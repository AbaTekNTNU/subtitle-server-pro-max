<script lang="ts">
  import type { Song } from "../types";
  import { Button } from "./ui/button";
  import Icon from "@iconify/svelte";

  const { songs, base }: { songs: Song[]; base: string } = $props();
</script>

<div class="flex w-full flex-col items-center gap-2">
  {#each songs as song}
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        class="w-44 rounded-lg p-4"
        onclick={() => {
          window.location.href = `/song/${song.id}`;
          fetch(`${base}/song/set`, {
            headers: {
              "Content-Type": "application/json",
            },
            method: "POST",
            body: JSON.stringify({ id: song.id }),
          });
        }}
      >
        <h2>{song.name}</h2>
      </Button>
      <a href={`edit/${song.id}`}>
        <Icon icon="solar:file-bold" class="scale-150" />
      </a>
    </div>
  {/each}
</div>
