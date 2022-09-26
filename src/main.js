const { invoke } = window.__TAURI__.tauri;

let word;
let wordDisplay;

let lives;

let wordElement;
let livesElement;
let guessElement;

window.addEventListener("DOMContentLoaded", () => {
    wordElement = document.querySelector("#word");
    livesElement = document.querySelector("#lives");
    guessElement = document.querySelector("#guess");

    getWord();
    lives = 6;
    livesElement.textContent = "Lives: " + lives;
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


    let isSolved = await invoke("is_solved", { wordDisplay: wordDisplay });
    if (lives > 0 && !isSolved) {

        let canGuess = await invoke("can_guess", { word: word, userGuess: guessElement.value });

        if (canGuess) {

            wordDisplay = await invoke("guess", { word: word, wordDisplay: wordDisplay, userGuess: guessElement.value });
            wordElement.textContent = wordDisplay;

        } else {

            lives -= 1;
            livesElement.textContent = "Lives: " + lives;

        }

        guessElement.value = "";

    }

}

async function guessKey(event) {

    if (event.key == "Enter") {

        guess();

    }

}

window.guess = guess;
window.guessKey = guessKey;