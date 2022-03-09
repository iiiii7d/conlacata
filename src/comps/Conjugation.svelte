<script lang="ts">
  import type { ConjugationObj, DimensionObj } from "src/_stores";
  import { writable, type Writable } from "svelte/store";
  import ContentEditable from "./ContentEditable.svelte";
  import Dimension from "./Dimension.svelte";

  export let conj: ConjugationObj;
  export let index: number;
  export let conjList: Writable<ConjugationObj[]>;
  let dimensions: Writable<DimensionObj[]>;
  $: dimensions = writable(conj.dimensions);

  function deleteConj() {
    $conjList.splice(index, 1);
    $conjList = $conjList;
  }

  function addDimension() {
    conj.dimensions = [...conj.dimensions, {
      name: "",
      description: "",
      rules: []
    }]
  }

</script>
<style lang="scss">
  @import "../_global";
  h2 {
    margin-bottom: 10px;
    margin-top: 0px;
  }
  div {
    display: inline-block;

    & > div {
      //border: 1px solid #aaa;
      //border-radius: 10px;
      padding: 5px;
      float: left;
      @include delete-button("& > i.fas.fa-trash");
    }
  }
</style>
<div>
  <div>
    <h2><ContentEditable bind:value={conj.name} placeholder="name"/></h2>
    <i>Description: <ContentEditable bind:value={conj.description} placeholder="description"/></i><br>
    <label for="isMultiDimensional">Multidimensional</label>
    <input type="checkbox" name="isMultiDimensional" bind:checked={conj.multiDimensional}>
    <button on:click={addDimension} disabled={!conj.multiDimensional}>
      <i class="fas fa-plus"></i> Add Dimension</button><br>
  </div>
  <div><i class="fas fa-trash" on:click={deleteConj}></i></div>
  {#if conj.multiDimensional}
    {#each $dimensions as dimension, i}
      <div><Dimension {dimension} {dimensions} multiDimensional={conj.multiDimensional}
        conjugationName={conj.name} index={i}/></div>
    {/each}
  {:else}
    <div><Dimension dimension={$dimensions[0]} {dimensions} multiDimensional={conj.multiDimensional}
      conjugationName={conj.name} index={0}/></div>
  {/if}
</div><br>
<hr>