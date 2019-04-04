import { diminish, determine_triad } from "./elegy";
console.log(diminish('C'));

const default_possible_notes = ['A', 'A#', 'B', 'C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#'];

//TODO: Allow user to change these
let allowed_chords = [
    'M', // Major Triad
    "m", // Minor Triad
    // "Diminished Triad",
    // "Augmented Triad",
];


let allowed_roots = [
    "C",
    // "C#",
    "Db",
    "D",
    // "D#",
    "Eb",
    "E",
    "F",
    // "F#",
    // "Gb",
    "G",
    // "G#",
    "Ab",
    "A",
    // "A#",
    "Bb",
    "B",
];

// let random_chord_type = "";
// let random_root = "";
// let requested_chord = "";
// let requested_notes = "";

let random_chord_type = allowed_chords[Math.floor(Math.random() * allowed_chords.length)];;
let random_root = allowed_roots[Math.floor(Math.random() * allowed_roots.length)];;
let requested_chord = random_root + random_chord_type;
let requested_notes = teoria.note(random_root).chord(random_chord_type).notes().toString().split(',');;
for (var i = 0; i < requested_notes.length; i++) {
    // Capitalize letter of each note and remove the octave number at the end
    requested_notes[i] = requested_notes[i].charAt(0).toUpperCase() + requested_notes[i].slice(1, -1);
}

// request_new_chord();
console.log(requested_chord);
console.log(requested_notes.join(","));

console.log(determine_triad(requested_notes));

// begin listening for midi input
navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);

console.log(1);
let chord_detected = false;
let notes = [];

// function request_new_chord() {
//     random_chord_type = allowed_chords[Math.floor(Math.random() * allowed_chords.length)];
//     random_root = allowed_roots[Math.floor(Math.random() * allowed_roots.length)];
//     requested_chord = random_root + random_chord_type;

//     requested_notes = teoria.note(random_root).chord(random_chord_type).notes().toString().split(',');
//     for (var i = 0; i < requested_notes.length; i++) {
//         // Capitalize letter of each note and remove the octave number at the end
//         requested_notes[i] = requested_notes[i].charAt(0).toUpperCase() + requested_notes[i].slice(1, -1);
//     }
// }

function determine_chord(key_numbers) {
    console.log("determining");
    var notes = [];
    if (key_numbers.length === 3) {
        for (var i = 0; i < key_numbers.length; i++) {
            var norm_index = (key_numbers[i] - 21) % 12
            notes[i] = norm_index;
        }

        console.log(notes);
    }
}

function onMIDISuccess(midiAccess) {
    console.log(2);
    for (var input of midiAccess.inputs.values()) {
        input.onmidimessage = getMIDIMessage;
    }
}

function onMIDIFailure() {
    console.log('Could not access your MIDI devices.');
}

function getMIDIMessage(message) {
    console.log(3);
    var command = message.data[0];
    var note = message.data[1];
    var velocity = (message.data.length > 2) ? message.data[2] : 0; // a velocity value might not be included with a noteOff command

    switch (command) {
    case 144: // noteOn
        if (velocity > 0) {
        noteOn(note, velocity);
        } else {
        noteOff(note);
        }
        break;
    case 128: // noteOff
        noteOff(note);
        break;
    // we could easily expand this switch statement to cover other types of commands such as controllers or sysex
    }
}

function noteOn(note) {
    console.log("on: " + note);
    notes.push(note);
    determine_chord(notes);
}

function noteOff(note) {
    console.log("off: " + note);
    notes = notes.filter(a => a !== note);
    determine_chord(notes);
}

// Fetch and instantiate our wasm module
// console.log("outside");
// fetch("elegy.wasm").then(response =>
//   response.arrayBuffer()
// ).then(bytes =>
//   WebAssembly.instantiate(bytes)
// ).then(results => {
//     console.log("inside");
//     let module = {};
//     let mod = results.instance;

//     navigator.requestMIDIAccess()
//       .then(onMIDISuccess, onMIDIFailure);
    
//     function onMIDISuccess(midiAccess) {
//       for (var input of midiAccess.inputs.values()) {
//           input.onmidimessage = getMIDIMessage;
//       }
//     }

//     function onMIDIFailure() {
//       console.log('Could not access your MIDI devices.');
//     }

//     function getMIDIMessage(message) {
//       var command = message.data[0];
//       var note = message.data[1];
//       var velocity = (message.data.length > 2) ? message.data[2] : 0; // a velocity value might not be included with a noteOff command

//       switch (command) {
//         case 144: // noteOn
//           if (velocity > 0) {
//             noteOn(note, velocity);
//           } else {
//             noteOff(note);
//           }
//           break;
//         case 128: // noteOff
//           noteOff(note);
//           break;
//         // we could easily expand this switch statement to cover other types of commands such as controllers or sysex
//       }
//     }

//     function noteOn(note) {
//       let answer = mod.exports.test1(note);
//       console.log("on: " + answer);
//     }

//     function noteOff(note) {
//       console.log("off: " + note);
//     }
// });