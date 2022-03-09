import { readable, writable } from "svelte/store";
import type {Writable} from "svelte/store";

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

export interface PartOfSpeechObj {
  name: string,
  description: string,
  abbrev: string,
  conjugations: Conjugation[]
}
export interface Conjugation {
  name: string,
  description: string,
  dimensional: boolean,
  dimensions: Dimension[],
}
export interface Dimension {
  name: string,
  description: string,
  rules: Map<RegExp, RegExp>,
}

export let lexicon = writable([] as WordObj[]);

export let partsOfSpeech = writable([] as PartOfSpeechObj[]);
export let globalPOS: Writable<PartOfSpeechObj> = writable({
  name: "",
  description: "",
  abbrev: "",
  conjugations: []
});
/*export let shiftKey = writable(false);

document.addEventListener("keydown", (e) => {
  shiftKey.set(true);
});
document.addEventListener("keyup", (e) => {
  shiftKey.set(false);
});
*/