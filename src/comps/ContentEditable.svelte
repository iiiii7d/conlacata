<style lang="scss">
  [contenteditable] {
    &:hover {
      text-decoration: underline;
      cursor: text;
    }
    &:focus {
      text-decoration: underline;
    }
    & + .fas {
      visibility: hidden !important;
    }
    &:hover + .fas {
      visibility: visible !important;
    }
    &:empty::before {
      content: attr(placeholder);
      color: grey;
    }
  }
</style>
<script>
  export let value;
  export let placeholder;
</script>
<span contenteditable placeholder={placeholder} bind:innerHTML={value}></span> <i class="fas fa-pen"></i>