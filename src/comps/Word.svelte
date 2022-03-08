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
    <h2><ContentEditable placeholderColor="gray" placeholder={`Word in ${$conName}`} bind:value={conWord}/></h2>
    <i><b>Pronunciaton:</b> <ContentEditable placeholder={"todo..."} bind:value={pronunciation}/></i><br>
    <b>Translation:</b> <ContentEditable placeholder="Base language translation..." bind:value={fromWord} /><br>
    <b>Part of speech:</b> TODO<br><br>
    <b>Description:</b> <ContentEditable placeholder="Description" bind:value={description} /><br>
  </div>
  <div>
    TODO conjugation table
  </div>
  <div><i class="fas fa-trash"></i></div>
</div><br>
<hr>