<script lang="ts">
  import type { ConjTableViewObj, ConjugationObj } from "src/_stores";


  export let value: ConjugationObj | undefined;
  export let axis: "x" | "y" | "z";
  export let conjugations: ConjugationObj[];
</script>
<label for={axis}>{axis} = </label><select name={axis} bind:value={value}>
  <option value={undefined}>(None)</option>
  {#each conjugations as conj}
    <option value={conj}>{conj.name}</option>
  {/each}
</select>