const { invoke } = window.__TAURI__.tauri;

let word;
let wordDisplay;

let lives;
let haveGuessed;

let wordElement;
let livesElement;
let guessElement;
let haveGuessedElement;

window.addEventListener("DOMContentLoaded", () => {
    wordElement = document.querySelector("#word");
    livesElement = document.querySelector("#lives");
    guessElement = document.querySelector("#guess");
    haveGuessedElement = document.querySelector("#haveGuessed");

    getWord();
    lives = 6;
    livesElement.textContent = "Lives: " + lives;

    haveGuessed = []
    displayGuessedLetters();
});

function hasGuessed(c) {

    for (let i = 0; i < haveGuessed.length; i++) {

        if (haveGuessed[i] == c) {

            return true;

        }

    }

    return false;

}

function displayGuessedLetters() {

    let display = "";
    for (let i = 0; i < haveGuessed.length; i++) {

        if (i < haveGuessed.length - 1) {

            display += haveGuessed[i] + ", ";

        } else {

            display += haveGuessed[i];

        }

    }

    haveGuessedElement.textContent = "Letters guessed: " + display;

}

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

        let canGuess = await invoke("can_guess", { word: word, userGuess: guessElement.value.toLowerCase() });

        if (canGuess) {

            wordDisplay = await invoke("guess", { word: word, wordDisplay: wordDisplay, userGuess: guessElement.value.toLowerCase() });
            wordElement.textContent = wordDisplay;

        } else {

            lives -= 1;
            livesElement.textContent = "Lives: " + lives;

        }


        if (!hasGuessed(guessElement.value.toLowerCase())) {

            haveGuessed.push(guessElement.value.toLowerCase());
            displayGuessedLetters();

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