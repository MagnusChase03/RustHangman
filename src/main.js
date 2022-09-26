const { invoke } = window.__TAURI__.tauri;

let word;
let wordDisplay;
let wordElement;
let guessElement;

window.addEventListener("DOMContentLoaded", () => {
    wordElement = document.querySelector("#word");
    guessElement = document.querySelector("#guess");

    getWord();
});

async function getWord() {

    word = await invoke("get_word");
    
    wordDisplay = "";
    for (let i = 0; i < word.length; i++) {
        
        if (i < word.length - 1) {

            wordDisplay += "_ ";

        } else {

            wordDisplay += "_";

        }
        
    }
    
    wordElement.textContent = wordDisplay;
}

async function guess() {

    wordDisplay = await invoke("guess", { word: word, wordDisplay: wordDisplay, userGuess: guessElement.value });
    wordElement.textContent = wordDisplay;

}

window.guess = guess;