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

      & > i.fas.fa-trash {
        color: red;
        padding: 2px;
        &:hover {
          background-color: #f008;
        }
        &:active {
          opacity: 1;
          color: white;
        }
      }
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
  {#each $dimensions as dimension, i}
    <div><Dimension {dimension} {dimensions} multiDimensional={conj.multiDimensional}
      conjugationName={conj.name} index={i}/></div>
  {/each}
</div><br>
<hr>