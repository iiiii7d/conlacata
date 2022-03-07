<script>
  import ContentEditable from "./ContentEditable.svelte";
  import {conName} from "../_stores";
  export let index;
  let conWord;
  let pronunciation;
  let fromWord;
  let description;

</script>
<style lang="scss">
  h2 {
    margin-bottom: 10px;
    margin-top: 0px;
  }
  div {
    border: 1px solid #aaa;
    border-radius: 10px;
    padding: 5px;
  }
</style>
<div>
  <h2><ContentEditable placeholder={`Word in ${$conName}`} bind:value={conWord}/></h2>
  <i>Pronunciaton: <ContentEditable placeholder={"todo..."} bind:value={pronunciation}/></i><br>
  Translation: <ContentEditable placeholder="Base language translation..." bind:value={fromWord} /><br>
  <ContentEditable placeholder="Description" bind:value={description} /><br>
</div>