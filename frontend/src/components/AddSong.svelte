<script lang="ts">
  import { Button } from "./ui/button";
  import { Textarea } from "./ui/textarea";
  import { Input } from "./ui/input";

  let form: HTMLFormElement;
  export let base: string;

  const handleSubmit = (e: Event) => {
    e.preventDefault();

    const formData = new FormData(form);
    const data = new URLSearchParams();
    for (let field of formData) {
      const [key, value] = field;
      data.append(key, value as string);
    }

    fetch(form.action, {
      method: form.method,
      body: data,
    }).catch((error) => {
      console.error("Error:", error);
    });
  };
</script>

<form
  method="post"
  action="{base}/song"
  class="flex w-full flex-col items-center gap-2"
  on:submit|preventDefault={handleSubmit}
  bind:this={form}
>
  <label for="name">Name:</label>
  <Input type="text" id="name" name="name" required class="w-1/3" />

  <div class="flex w-1/2 flex-col items-center">
    <label for="lines" class="self-start">Lines:</label>
    <Textarea id="lines" name="lines" class="h-96 w-full" />

    <Button on:click={handleSubmit} class="self-start">Add Song</Button>
  </div>
</form>
