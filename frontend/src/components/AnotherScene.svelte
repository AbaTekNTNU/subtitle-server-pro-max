<script lang="ts">
  import { T } from "@threlte/core";
  import { Text3DGeometry, Align } from "@threlte/extras";
</script>

<T.PerspectiveCamera
  makeDefault
  position={[0, 1, 20]}
  fov={15}
  oncreate={(e) => {
    e.lookAt(0, 1, 0);
  }}
/>
<T.DirectionalLight intensity={0.8} position.x={5} position.y={10} />
<T.AmbientLight intensity={0.2} />

