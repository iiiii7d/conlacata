<script lang="ts">
  import type {ConjugationObj } from "../_stores";


  export let value: string | undefined;
  export let axis: "x" | "y" | "z";
  export let conjugations: ConjugationObj[];
  export let otherSelected: (string | undefined)[]
</script>
<label for={axis}>{axis} = </label><select name={axis} bind:value={value}>
  <option value={undefined}>(None)</option>
  {#each conjugations.filter(conj => conj.name == undefined || !otherSelected.includes(conj.name)) as conj}
    <option value={conj.name}>{conj.name}</option>
  {/each}
</select>