<script lang="ts">
  import { onMount } from "svelte";

  interface Props {
    base: string;
  }

  let { base }: Props = $props();

  let line = $state("");

  onMount(() => {
    let ev = new EventSource(`${base}/sse`);

    ev.onmessage = (event) => {
      line = event.data;
    };

    return () => ev.close();
  });
</script>

<h1>{line}</h1>
