<script lang="ts">
  import type { LineComp } from "$lib/bindings/LineComp";
  import Icon from "@iconify/svelte";
  import { Button } from "./ui/button";
  import * as Dialog from "./ui/dialog";
  import { Input } from "./ui/input";
  import { Label } from "./ui/label";
  import { textToVector } from "$lib/utils";
  import { toast } from "svelte-sonner";

  type Props = {
    lines: LineComp[];
    url: string;
  };

  const { lines, url }: Props = $props();

  let current_line = $state(lines[0]);

  const fetchLines = async (id: number) => {
    const res = await fetch(`${url}/edit/line?id=${id}`);
    const data = await res.json();
    current_line = data;
  };

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const id = form.line_id.value;
    const position = textToVector(form.position.value);
    const cam_position = textToVector(form.cam_position.value);
    const cam_look_at = textToVector(form.cam_look_at.value);
    const color = form.color.value === "" ? null : form.color.value;
    const keep_n_last =
      form.keep_n_last.value === "" ? 0 : form.keep_n_last.value;
    const end_position = textToVector(form.end_position.value);
    const cam_end_position = textToVector(form.cam_end_position.value);
    const cam_end_look_at = textToVector(form.cam_end_look_at.value);

    const comp: LineComp = {
      id: Number(id),
      line: form.line.value,
      position: position!,
      cam_position: cam_position!,
      cam_look_at: cam_look_at!,
      rotation: textToVector(form.rotation.value),
      color: color ?? null,
      keep_n_last: keep_n_last ? Number(keep_n_last) : 0,
      end_position: end_position ?? null,
      cam_end_position: cam_end_position ?? null,
      cam_end_look_at: cam_end_look_at ?? null,
      cam_rotation: textToVector(form.camera_rotation.value),
      text_animation: null,
    };

    const res = await fetch(`${url}/song/edit`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(comp),
    });

    if (res.ok) {
      console.log("success");
      toast.success("Line updated");
      await fetchLines(Number(id));
    } else {
      console.error("error");
    }
  };

  const deleteLine = async (id: number) => {
    const res = await fetch(`${url}/song/edit`, {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ id: id }),
    });

    if (res.ok) {
      console.log("success");
    } else {
      console.error("error");
    }
  };
</script>

<div class="flex h-dvh w-full">
  <div
    class="flex w-1/2 flex-col items-center gap-1 overflow-y-scroll pb-24 pt-12"
  >
    {#each lines as line}
      {#if line.id === current_line.id}
        <div
          class="grid w-1/2 grid-cols-4 items-center gap-2 bg-primary-foreground"
        >
          <Button variant="outline" class={"col-span-1 justify-self-end"}>
            <Icon icon="akar-icons:edit" class="scale-150" />
          </Button>
          <p class="col-span-3 text-left">{line.line}</p>
        </div>
      {:else}
        <div class="grid w-1/2 grid-cols-4 items-center gap-2">
          <Button
            variant="outline"
            class={"col-span-1 justify-self-end"}
            onclick={() => {
              fetchLines(line.id);
            }}
          >
            <Icon icon="akar-icons:edit" class="scale-150" />
          </Button>
          <p class="col-span-3 text-left">{line.line}</p>
        </div>
      {/if}
    {/each}
  </div>
  <div class="flex w-1/2 justify-center overflow-y-scroll pb-24 pt-12">
    <form onsubmit={handleSubmit}>
      <input type="hidden" name="line_id" value={current_line.id} />
      <div class="grid gap-4 py-4">
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="line" class="text-right">Line</Label>
          <Input
            id="line"
            type="text"
            value={current_line.line}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="position" class="text-right">Position</Label>
          <Input
            id="position"
            type="text"
            value={`${current_line.position.x} ${current_line.position.y} ${current_line.position.z}`}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="cam_position" class="text-right">Cam position</Label>
          <Input
            id="cam_position"
            type="text"
            value={`${current_line.cam_position.x} ${current_line.cam_position.y} ${current_line.cam_position.z}`}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="cam_look_at" class="text-right">Cam look at</Label>
          <Input
            id="cam_look_at"
            type="text"
            value={`${current_line.cam_look_at.x} ${current_line.cam_look_at.y} ${current_line.cam_look_at.z}`}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="rotation" class="text-right">rotation</Label>
          <Input
            id="rotation"
            type="text"
            value={current_line.rotation
              ? `${current_line.rotation.x} ${current_line.rotation.y} ${current_line.rotation.z}`
              : "null"}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="camera_rotation" class="text-right">Camera rotation</Label
          >
          <Input
            id="camera_rotation"
            type="text"
            value={current_line.cam_rotation
              ? `${current_line.cam_rotation.x} ${current_line.cam_rotation.y} ${current_line.cam_rotation.z}`
              : "null"}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="color" class="text-right">color</Label>
          <Input
            id="color"
            type="text"
            value={current_line.color}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="keep_n_last" class="text-right">Keep n last</Label>
          <Input
            id="keep_n_last"
            type="text"
            value={`${current_line.keep_n_last}`}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="end_position" class="text-right">End position</Label>
          <Input
            id="end_position"
            type="text"
            value={current_line.end_position
              ? `${current_line.end_position.x} ${current_line.end_position.y} ${current_line.end_position.z}`
              : "null"}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="cam_end_position" class="text-right"
            >Cam end position</Label
          >
          <Input
            id="cam_end_position"
            type="text"
            value={`${current_line.cam_end_position}`}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="cam_end_look_at" class="text-right">Cam end lookat</Label>
          <Input
            id="cam_end_look_at"
            type="text"
            value={`${current_line.cam_end_look_at}`}
            class="col-span-3 bg-primary-foreground"
          />
        </div>
      </div>
      <Dialog.Footer class="flex w-full justify-around">
        <Button
          type="button"
          variant="outline"
          onclick={() => deleteLine(current_line.id)}>Delete</Button
        >
        <Button type="submit">Save changes</Button>
      </Dialog.Footer>
    </form>
  </div>
</div>
