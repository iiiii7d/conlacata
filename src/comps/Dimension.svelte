<script lang="ts">
  import type { DimensionObj, RuleObj } from "src/_stores";
  import { writable, type Writable } from "svelte/store";
  import ContentEditable from "./ContentEditable.svelte";
import Rule from "./Rule.svelte";

  export let dimension: DimensionObj;
  export let index: number;
  export let dimensions: Writable<DimensionObj[]>;
  export let multiDimensional: boolean;
  export let conjugationName: string;
  let rules: Writable<RuleObj[]>;
  $: rules = writable(dimension.rules);

  function deleteDimension() {
    if ($dimensions.length <= 1) return;
    $dimensions.splice(index, 1);
    $dimensions = $dimensions;
  }

  function addRule() {
    dimension.rules = [...dimension.rules, {regex: new RegExp(""), subst: ""}];
  }

</script>
<style lang="scss">
  @import "../global";
  h3 {
    margin-bottom: 0px;
    margin-top: 0px;
  }
  @include delete-button("div i.fas.fa-trash");
</style>
<div>
  {#if multiDimensional}
    <h3><ContentEditable placeholder={index == 0 ? "(Default)" : "name"}
      disabled={!multiDimensional} bind:value={dimension.name} />
    <i class="fas fa-trash" on:click={deleteDimension}></i></h3>
    {#if index != 0}
      <ContentEditable placeholder="description" bind:value={dimension.description} /><br>
    {/if}
  {/if}
  {#each $rules as rule, i}
    <Rule {rule} {rules} index={i}/>
  {/each}
  <button on:click={addRule}><i class="fas fa-plus"></i> Add Rule</button>
</div>