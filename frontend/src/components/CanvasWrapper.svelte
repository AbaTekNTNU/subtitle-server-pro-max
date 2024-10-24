<script lang="ts">
  import { Canvas } from "@threlte/core";
  import Text from "./Text.svelte";

  interface Props {
    base: string;
  }

  const { base }: Props = $props();
</script>

<Canvas>
  <Text {base} />
</Canvas>
