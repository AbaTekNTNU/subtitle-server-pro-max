<script lang="ts">
  import { Button } from "./ui/button";
  import { Textarea } from "./ui/textarea";
  import { Input } from "./ui/input";
  import { toast } from "svelte-sonner";
  type Props = {
    base: string;
  };

  let form: HTMLFormElement;
  const { base }: Props = $props();

  const handleSubmit = async (e: Event) => {
    e.preventDefault();

    const formData = new FormData(form);
    const data = new URLSearchParams();
    for (let field of formData) {
      const [key, value] = field;
      data.append(key, value as string);
    }

    const res = await fetch(form.action, {
      method: form.method,
      body: data,
    });

    if (res.ok) {
      toast.success("Song added successfully");
      form.reset();
    }
  };
</script>

<form
  method="post"
  action="{base}/song"
  class="flex w-full flex-col items-center gap-2"
  onsubmit={handleSubmit}
  bind:this={form}
>
  <label for="name">Name:</label>
  <Input type="text" id="name" name="name" required class="w-1/3" />

  <div class="flex w-1/2 flex-col items-center">
    <label for="lines" class="self-start">Lines:</label>
    <Textarea id="lines" name="lines" class="h-96 w-full" />

    <Button onclick={handleSubmit} class="self-start">Add Song</Button>
  </div>
</form>
