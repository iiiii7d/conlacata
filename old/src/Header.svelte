<script lang="ts">
  import {pageName} from "./_stores";
</script>
<style>
  header {
    background-color: #eee;
    padding: 0px 0px 10px 0px;
    transform: translateX(50px);
    width: calc(100vw - 50px);
  }
  header > h1 {
    font-size: 50px;
    padding: 0px;
    margin: 0px;
    padding-left: 50px;
  }
</style>

<header><br><h1>{$pageName}</h1><br></header>