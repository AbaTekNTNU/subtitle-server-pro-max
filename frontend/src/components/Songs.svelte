<script lang="ts">
  import type { Song } from "../types";
  import { Button } from "./ui/button";

  const { songs, base }: { songs: Song[]; base: string } = $props();
</script>

<div class="flex w-full flex-col items-center gap-2">
  {#each songs as song}
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
  {/each}
</div>
