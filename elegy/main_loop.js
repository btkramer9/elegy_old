import { determine_triad } from "./elegy";

const default_possible_notes = ['A', 'A#', 'B', 'C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#'];
const chord_shorthand = { 'M':'Major', 'm':'Minor', 'dim':'Diminished', 'aug':'Augmented' };

$( '#chord-select a' ).on( 'click', function( event ) {
    var $target = $(event.currentTarget ),
    val = $target.attr( 'data-value' ),
    $inp = $target.find( 'input' ),
    idx;

    if ( ( idx = allowed_chords.indexOf(val) ) > -1 ) {
        allowed_chords.splice( idx, 1 );
        setTimeout( function() { $inp.prop( 'checked', false ) }, 0);
    } else {
        allowed_chords.push( val );
        setTimeout( function() { $inp.prop( 'checked', true) }, 0);
    }

    $( event.target ).blur();
    return false
})

$( '#root-select a' ).on( 'click', function( event ) {
    var $target = $(event.currentTarget ),
    val = $target.attr( 'data-value' ),
    $inp = $target.find( 'input' ),
    idx;

    if ( ( idx = allowed_roots.indexOf(val) ) > -1 ) {
        allowed_roots.splice( idx, 1 );
        setTimeout( function() { $inp.prop( 'checked', false ) }, 0);
    } else {
        allowed_roots.push( val );
        setTimeout( function() { $inp.prop( 'checked', true) }, 0);
    }

    $( event.target ).blur();
    return false
})

$('.chord-display-format').on('click', function () {
    // console.log($('.chord-display-format').not(this).prop( 'checked', false ));
    chord_format = $(this).attr( 'data-value' );
});

let chord_format = "";
let allowed_chords = [];
let allowed_roots = [];
$('.default-on').each(function() {
    $( this ).trigger('click');
});

let notes_hint = false;
let notes = [];
let random_chord_type = "";
let random_root = "";
let requested_chord = "";
let requested_notes = "";
let possible_notes = [];
request_new_chord();

// begin listening for midi input
navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);

$('#notes-hint').click(function() {
    var active = $(this).hasClass('active');
    if (active) {
        $(this).removeClass('active').addClass('disabled');
        notes_hint = false;
    } else {
        $(this).removeClass('disabled').addClass('active');
        notes_hint = true;
    }
    showRequestedChord(notes_hint);
})

function determine_chord(key_numbers) {
    var notes = [];
    if (key_numbers.length === 3) {
        key_numbers = key_numbers.sort(function(a,b){return a - b})

        for (var i = 0; i < key_numbers.length; i++) {
            var norm_index = (key_numbers[i] - 21) % 12
            notes[i] = norm_index;
        }
        
        for (var i = 0; i < notes.length; i++) {
            notes[i] = possible_notes[notes[i]];
        }


        // console.log("Current Notes: " + notes)
        var attempted_notes = notes.join(',');
        var attempted_answer = determine_triad(attempted_notes);
        var attempted_chords = attempted_answer.split(',');
        var correct_chord_detected = attempted_chords.includes(requested_chord)
        if (correct_chord_detected) {
            // console.log("Correct! " + attempted_answer);

            // Move to the history window
            var ul = document.getElementById("history-list");
            var li = document.createElement("li");
            li.appendChild(document.createTextNode(requested_chord));
            ul.appendChild(li);

            // Clear attempted chords
            $('#previous-attempts').empty();

            // Generate a new chord
            request_new_chord();
        }
        else {
            console.log("Try Again. " + requested_chord + " not in " + attempted_chords);
            var ul = document.getElementById("previous-attempts");
            var li = document.createElement("li");
            li.appendChild(document.createTextNode(attempted_chords));
            ul.appendChild(li);
        }
    }
}

function onMIDISuccess(midiAccess) {
    for (var input of midiAccess.inputs.values()) {
        input.onmidimessage = getMIDIMessage;
    }
}

function onMIDIFailure() {
    console.log('Could not access your MIDI devices.');
}

function getMIDIMessage(message) {
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
        break
    default:
        console.log("unknown command: [" + command + " " + note + " " + velocity + "]");        
    // we could easily expand this switch statement to cover other types of commands such as controllers or sysex
    }
}

function noteOn(note) {
    console.log("requested_chord: " + requested_chord);
    // console.log("on: " + note);
    notes.push(note);
    determine_chord(notes);
}

function noteOff(note) {
    // console.log("off: " + note);
    notes = notes.filter(a => a !== note);
    determine_chord(notes);
}

function showRequestedChord(notes_hint) {
    var final_request = requested_chord;
    console.log("final_request: " + final_request);
    console.log("chord_format: " + chord_format);
    console.log("notes_hint: " + notes_hint);
    if (chord_format === 'expanded') {
        console.log("random_root: " + random_root);
        console.log("chord_shorthand[random_chord_type]: " + chord_shorthand[random_chord_type]);
        final_request = random_root + " " + chord_shorthand[random_chord_type];
        console.log("final_request: " + final_request);
    }

    if (!notes_hint) {
        document.getElementById( 'requested-chord' ).innerText = final_request;
    } else {
        document.getElementById( 'requested-chord' ).innerText = final_request + " (" + requested_notes + ")";
    }
    
}

function request_new_chord() {
    random_chord_type = allowed_chords[Math.floor(Math.random() * allowed_chords.length)];
    random_root = allowed_roots[Math.floor(Math.random() * allowed_roots.length)];
    requested_chord = random_root + random_chord_type;

    requested_notes = ["C", "E", "G"]; //teoria.note(random_root).chord(random_chord_type).notes().toString().split(',');
    for (var i = 0; i < requested_notes.length; i++) {
        // Capitalize letter of each note and remove the octave number at the end
        requested_notes[i] = requested_notes[i].charAt(0).toUpperCase() + requested_notes[i].slice(1, -1);
    }
    possible_notes = generatePossibleNotes(requested_notes);

    console.log("play: " + requested_chord);
    showRequestedChord(notes_hint);
    console.log("answer: " + requested_notes.join(","));
    console.log("Possible notes: " + possible_notes);
}

function generatePossibleNotes(requested_notes) {
    // reset possible_notes to the default sharps and roots
    for (var i = 0; i < default_possible_notes.length; i++) {
        possible_notes[i] = default_possible_notes[i];
    }
    for (var i = 0; i < requested_notes.length; i++) {
        var note = requested_notes[i];
        var note_root = note.slice(0,1);
        var accidental = note.slice(1);
        if (accidental.length > 0) {
            switch (accidental) {
                case "b":
                    switch (note_root) {
                        case "A":
                            possible_notes[11] = "Ab";
                            break;
                        case "B":
                            possible_notes[1] = "Bb";                            
                            break;
                        case "C":
                            possible_notes[2] = "Cb";
                            break;
                        case "D":
                            possible_notes[4] = "Db";
                            break;
                        case "E":
                            possible_notes[6] = "Eb";                            
                            break;
                        case "F":
                            possible_notes[7] = "Fb";
                            break;
                        case "G":
                            possible_notes[9] = "Gb";                            
                            break;
                    }
                    break;
                case "#":
                    switch (note_root) {
                        case "A":
                            possible_notes[3] = "B#";
                            break;
                        case "B":
                            possible_notes[8] = "E#";                            
                            break;
                    }
                case "bb":
                    // TODO: add double flat cases
                    break;
                case "##":
                    // TODO: add double sharp cases
                    break;
            }
        }
    }

    return possible_notes;
}