<script lang="ts">
  import {pageName} from "./_stores";
  
  const pages = {
    "Home": "door-open",
    "Characters": "a",
    "Lexicon": "book"
  };

  var menuOpen = false;

  function loadPage(page: string) {
    pageName.set(page);
  }


</script>
<style lang="scss">
  @import "./_global";
  nav {
    width: fit-content;
    min-width: 50px;
    height: 100vh;
    margin: 0;
    padding: 0;
    background-color: $sidebar;
    display: block;
    float: left;
    text-align: center;
    position: fixed;
    z-index: 1;
  }
  nav.selected {
    text-align: left;
    box-shadow: 5px 0px 5px $sidebar-shadow;
  }
  nav > div {
    margin: 0;
    width: calc(100% - 30px);
    padding: 15px;
    color: black;
    display: inline-block;
  }
  nav > div:hover {
    background-color: $sidebar-hover;
    cursor: pointer;
  }
  nav > div:active {
    background-color: $sidebar-active;
  }
  nav > div > span {
    padding: 0px 0px 0px 10px;
    display: none;
    @include unclickable;
  }
  nav.selected > div > span {
    display: inline-block;
  }

  nav > hr {
    margin: 0px;
  }
</style>
<nav on:mouseleave={() => menuOpen = false} class:selected={menuOpen}>
  <div on:click={() => menuOpen = true}><i class="fas fa-bars"></i><span><b>Conlacata</b></span></div><br>
  {#each Object.entries(pages) as [page, icon]}
    <div on:click={() => loadPage(page)}><i class={"fas fa-"+icon}></i><span>{page}</span></div><br>
  {/each}
</nav>