<script lang="ts">
  import type { RuleObj } from "../_stores";
  import type { Writable } from "svelte/store";


  export let rule: RuleObj;
  export let rules: Writable<RuleObj[]>;
  export let index: number;
  let regexError: boolean
  let preRegex = rule.regex.toString().slice(1, rule.regex.toString().length-1);
  $: try {
    rule.regex = new RegExp(preRegex);
    regexError = false;
  }
  catch (SyntaxError) {
    regexError = true;
  }

  function deleteRule() {
    $rules.splice(index, 1);
    $rules = $rules;
  }
</script>
<style lang="scss">
  i.fas.fa-trash {
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
  input.error {
    color: red;
  }
</style>
<input type="text" bind:value={preRegex} class:error={regexError} size="5">
<i class="fas fa-chevron-right"></i>
<input type="text" bind:value={rule.subst} size="5">
<i class="fas fa-trash" on:click={deleteRule}></i><br>