<script lang="ts">
  import type { ConjugationObj } from "../_stores";


  export let values: [string, string][];
  export let conjugations: ConjugationObj[];
  export let otherSelected: (string | undefined)[]
</script>
<label for="Global">Global = </label><select name="Global" multiple bind:value={values}>
  {#each conjugations.filter(conj => conj.name == undefined || !otherSelected.includes(conj.name)) as conj}
    {#if conj.multiDimensional}
    <optgroup label={conj.name}>
      {#each conj.dimensions as dim}
        <option value={[conj.name, dim.name]}>{dim.name}{values.includes([conj.name, dim.name]) ? " <<" : ""}</option>
      {/each}
    </optgroup>
    {:else}
      <option value={[conj.name, ""]}>{conj.name}{values.includes([conj.name, ""]) ? " <<" : ""}</option>
    {/if}
  {/each}
</select>