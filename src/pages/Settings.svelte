<script lang="ts">
import LZString from "lz-string";


  import ContentEditable from "../comps/ContentEditable.svelte";
  import { conName } from "../_stores";

  let exporter: HTMLAnchorElement;
  let importer: HTMLInputElement;

  function b64EncodeUnicode(str: string) {
    return btoa(encodeURIComponent(str).replace(/%([0-9A-F]{2})/g, function(match, p1) {
        return String.fromCharCode(parseInt(p1, 16))
    }))
  }

  function b64DecodeUnicode(str: string) {
      return decodeURIComponent(Array.prototype.map.call(atob(str), function(c) {
          return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2)
      }).join(''))
  }

  function exportData() {
    let dataStr = "data:text/plain;charset=utf-8," + b64EncodeUnicode(LZString.decompress(localStorage.conlacata) as string);
    exporter.href = dataStr;
    exporter.download = $conName+".clct";
    exporter.click();
  }
  function importData() {
    // @ts-ignore
    let importedFile = importer.files[0];

    let reader = new FileReader();
    reader.onload = () => {
      if (!confirm("Are you sure you want to irreversibly replace all data with the import?")) return;
      let fileContent = reader.result;
      importer.value = "";
      localStorage.conlacata = LZString.compress(b64DecodeUnicode(fileContent as string));
      location.reload();
    };
    reader.readAsText(importedFile); 
  }
  function wipeData() {
    if (confirm("Are you ure you want to irreversibly wipe all data?")) {
      localStorage.conlacata = undefined;
      location.reload();
    }
  }

</script>
Language name: <ContentEditable placeholder="Foobar" bind:value={$conName} />
<button on:click={exportData}>Export data</button>
<button on:click={() => importer.click()}>Import data</button>
<button on:click={wipeData}>Wipe data</button>
<a bind:this={exporter} id="downloader" style="display:none"> </a>
<input bind:this={importer} type="file" name="importer"
  id="importer" accept=".clct" style="display:none" on:input={importData}>