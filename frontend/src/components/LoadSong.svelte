<script lang="ts">
  import Button from "./ui/button/button.svelte";

  type Props = {
    base: string;
    id: number;
  };

  const { base, id }: Props = $props();
</script>

<Button
  onclick={() => {
    fetch(`${base}/song/set`, {
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
      body: JSON.stringify({ id: Number(id) }),
    });
  }}>Load Song</Button
>
