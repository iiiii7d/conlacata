import { readable, writable } from "svelte/store";

export let pageName = writable("Home");
export const version = readable("v0.0.0");

export let conName = writable("Foobar");

export interface CharacterObj {
  char: string,
  ipa: string,
}
export let characters = writable([] as CharacterObj[]);
export let otherCharacters = writable([] as CharacterObj[]);

export interface WordObj {
  conWord: string,
  fromWord: string,
  description: string,
  pronunciation: string,
  partOfSpeech: string,
  derivedWords: WordObj[],
  tags: string[],
  synonyms: string[],
  antonyms: string[]
}
export let lexicon = writable([] as WordObj[]);
/*export let shiftKey = writable(false);

document.addEventListener("keydown", (e) => {
  shiftKey.set(true);
});
document.addEventListener("keyup", (e) => {
  shiftKey.set(false);
});
*/