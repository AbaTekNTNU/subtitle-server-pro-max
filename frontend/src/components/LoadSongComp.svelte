<script lang="ts">
  import Icon from "@iconify/svelte";
  import Button from "./ui/button/button.svelte";
  import { toast } from "svelte-sonner";

  type Props = {
    base: string;
    id: number;
  };

  const { base, id }: Props = $props();

  const handleLoad = async () => {
    const res = await fetch(`${base}/song/set`, {
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
      body: JSON.stringify({ id: Number(id) }),
    });

    if (res.ok) {
      toast.success("Song load dispatched");
    }
  };
</script>

<div class="flex items-center gap-2">
  <a href={`/edit/${id}`}>
    <Icon icon="solar:file-bold" class="scale-150" />
  </a>
  <Button onclick={handleLoad}>Load Song</Button>
</div>
