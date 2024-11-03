<script lang="ts">
  import type { LineComp } from "$lib/bindings/LineComp";
  import Icon from "@iconify/svelte";
  import { Button, buttonVariants } from "./ui/button";
  import * as Dialog from "./ui/dialog";
  import { Input } from "./ui/input";
  import { Label } from "./ui/label";
  import { cn, textToVector } from "$lib/utils";

  type Props = {
    lines: LineComp[];
    url: string;
  };

  const { lines, url }: Props = $props();

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
      color: color ?? null,
      keep_n_last: keep_n_last ? Number(keep_n_last) : 0,
      end_position: end_position ?? null,
      cam_end_position: cam_end_position ?? null,
      cam_end_look_at: cam_end_look_at ?? null,
    };

    console.log(comp);

    const res = await fetch(`${url}/song/edit`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(comp),
    });

    if (res.ok) {
      console.log("success");
      location.reload();
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
      location.reload();
    } else {
      console.error("error");
    }
  };
</script>

<div class="flex w-full flex-col items-center gap-1 pb-24">
  {#each lines as line}
    <div class="grid w-1/2 grid-cols-4 items-center gap-2">
      <Dialog.Root>
        <Dialog.Trigger
          class={cn(
            buttonVariants({ variant: "outline", size: "icon" }),
            "col-span-1 justify-self-end",
          )}
        >
          <Icon icon="akar-icons:edit" class="scale-150" />
        </Dialog.Trigger>
        <Dialog.Content class="sm:max-w-[425px]">
          <Dialog.Header>
            <Dialog.Title>Edit position</Dialog.Title>
            <Dialog.Description>
              Make changes as position and camera position. x y z are space
              separated values.
            </Dialog.Description>
          </Dialog.Header>
          <form onsubmit={handleSubmit}>
            <input type="hidden" name="line_id" value={line.id} />
            <div class="grid gap-4 py-4">
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="line" class="text-right">Line</Label>
                <Input
                  id="line"
                  type="text"
                  value={line.line}
                  class="col-span-3"
                />
              </div>
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="position" class="text-right">Position</Label>
                <Input
                  id="position"
                  type="text"
                  value={`${line.position.x} ${line.position.y} ${line.position.z}`}
                  class="col-span-3"
                />
              </div>
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="cam_position" class="text-right">Cam position</Label
                >
                <Input
                  id="cam_position"
                  type="text"
                  value={`${line.cam_position.x} ${line.cam_position.y} ${line.cam_position.z}`}
                  class="col-span-3"
                />
              </div>
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="cam_look_at" class="text-right">Cam look at</Label>
                <Input
                  id="cam_look_at"
                  type="text"
                  value={`${line.cam_look_at.x} ${line.cam_look_at.y} ${line.cam_look_at.z}`}
                  class="col-span-3"
                />
              </div>
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="color" class="text-right">color</Label>
                <Input
                  id="color"
                  type="text"
                  value={line.color}
                  class="col-span-3"
                />
              </div>
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="keep_n_last" class="text-right">Keep n last</Label>
                <Input
                  id="keep_n_last"
                  type="text"
                  value={`${line.keep_n_last}`}
                  class="col-span-3"
                />
              </div>
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="end_position" class="text-right">End position</Label
                >
                <Input
                  id="end_position"
                  type="text"
                  value={`${line.end_position}`}
                  class="col-span-3"
                />
              </div>
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="cam_end_position" class="text-right"
                  >Cam end position</Label
                >
                <Input
                  id="cam_end_position"
                  type="text"
                  value={`${line.cam_end_position}`}
                  class="col-span-3"
                />
              </div>
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="cam_end_look_at" class="text-right"
                  >Cam end lookat</Label
                >
                <Input
                  id="cam_end_look_at"
                  type="text"
                  value={`${line.cam_end_look_at}`}
                  class="col-span-3"
                />
              </div>
            </div>
            <Dialog.Footer class="flex w-full justify-around">
              <Button
                type="button"
                variant="outline"
                onclick={() => deleteLine(line.id)}>Delete</Button
              >
              <Button type="submit">Save changes</Button>
            </Dialog.Footer>
          </form>
        </Dialog.Content>
      </Dialog.Root>
      <p class="col-span-3 text-left">{line.line}</p>
    </div>
  {/each}
</div>
