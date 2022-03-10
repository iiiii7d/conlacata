import { readable, writable } from "svelte/store";
import type {Writable} from "svelte/store";
import LZString from "lz-string";

export interface localStorageFormat {
  pageName: string,
  conName: string,
  characters: CharacterObj[]
  otherCharacters: CharacterObj[]
  lexicon: WordObj[]
  partsOfSpeech: PartOfSpeechObj[],
  globalPOS: PartOfSpeechObj
}
export const defaultLocalStorageFormat = (): localStorageFormat => ({...{
  pageName: "Home",
  conName: "Foobar",
  characters: [],
  otherCharacters: [],
  lexicon: [],
  partsOfSpeech: [],
  globalPOS: defaultPartOfSpeech()
}});

export interface CharacterObj {
  char: string,
  ipa: string,
}
export const defaultCharacter = (): CharacterObj => ({...{
  char: "",
  ipa: ""
}});

export interface WordObj {
  conWord: string,
  fromWord: string,
  description: string,
  pronunciation: string,
  partOfSpeech?: string,
  derivedWords: WordObj[],
  tags: string[],
  synonyms: string[],
  antonyms: string[]
}
export const defaultWord = (): WordObj => ({...{
  conWord: "",
  fromWord: "",
  description: "",
  pronunciation: "",
  partOfSpeech: undefined,
  derivedWords: [],
  tags: [],
  synonyms: [],
  antonyms: []
}});

export interface PartOfSpeechObj {
  name: string,
  description: string,
  abbrev: string,
  conjugations: ConjugationObj[]
  conjTableView: ConjTableViewObj,
}
export const defaultPartOfSpeech = (): PartOfSpeechObj => ({...{
  name: "",
  description: "",
  abbrev: "",
  conjugations: [],
  conjTableView: {},
}});
export interface ConjugationObj {
  name: string,
  description: string,
  multiDimensional: boolean,
  dimensions: DimensionObj[],
}
export const defaultConjugation = (): ConjugationObj => ({...{
  name: "",
  description: "",
  multiDimensional: true,
  dimensions: [defaultDimension()]
}});
export interface DimensionObj {
  name: string,
  description: string,
  rules: RuleObj[],
}
export const defaultDimension = (): DimensionObj => ({...{
  name: "",
  description: "",
  rules: []
}})
export interface RuleObj {
  regex: string,
  subst: string
}
export const defaultRule = (): RuleObj => ({...{
  regex: "(?:)",
  subst: ""
}})
export interface ConjTableViewObj {
  x?: ConjugationObj,
  y?: ConjugationObj,
  z?: ConjugationObj
}
/*export let shiftKey = writable(false);

document.addEventListener("keydown", (e) => {
  shiftKey.set(true);
});
document.addEventListener("keyup", (e) => {
  shiftKey.set(false);
});
*/

let loadedLocalStorage: localStorageFormat = JSON.parse(
    LZString.decompress(localStorage.conlacata
      ?? LZString.compress(JSON.stringify(defaultLocalStorageFormat())))
   ?? JSON.stringify(defaultLocalStorageFormat()));
// @ts-ignore
if (loadedLocalStorage == {}) loadedLocalStorage = defaultLocalStorageFormat();

export let pageName = writable(loadedLocalStorage.pageName
  ?? defaultLocalStorageFormat().pageName);
export const version = readable("v0.0.0");

export let conName = writable(loadedLocalStorage.conName
  ?? defaultLocalStorageFormat().conName);

export let characters = writable(loadedLocalStorage.characters
  ?? defaultLocalStorageFormat().characters);
export let otherCharacters = writable(loadedLocalStorage.otherCharacters
  ?? defaultLocalStorageFormat().otherCharacters);

export let lexicon = writable(loadedLocalStorage.lexicon
  ?? defaultLocalStorageFormat().lexicon);

export let partsOfSpeech = writable(loadedLocalStorage.partsOfSpeech
  ?? defaultLocalStorageFormat().partsOfSpeech);
export let globalPOS: Writable<PartOfSpeechObj> = writable(loadedLocalStorage.globalPOS
  ?? defaultLocalStorageFormat().globalPOS);