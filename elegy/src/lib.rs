#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

#[wasm_bindgen]
pub fn diminish(note: &str) -> String {
    notes::diminish(note)
}

#[wasm_bindgen]
pub fn determine_triad(mut notes: &str) -> String {
    // easiest to pass notes as single string through wasm layer
    // and split here
    let triad = notes.split(",").collect::<Vec<&str>>();
    let result_vec = chords::determine_triad(triad);

    // easier to return as one string here aswell
    result_vec.join(",").to_string()
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}


mod notes {
    pub const FIFTHS: &[char; 7] = &['C', 'G', 'D', 'A', 'E', 'B', 'F'];

    pub fn diminish(note: &str) -> String {
        let mut result = note.to_string();
        // get last character without modifying the original
        let last_char = result.clone().pop().unwrap();

        if last_char != '#' {
            // append a flat
            return result + "b"
        } else {
            // remove last sharp
            result.pop();
            return result
        }
    }

    pub fn augment(note: &str) -> String {
        let mut result = note.to_string();
        // get last character without modifying the original
        let last_char = result.clone().pop().unwrap();

        if last_char != 'b' {
            // append a sharp
            return result + "#"
        } else {
            // remove last flat
            result.pop();
            return result
        }
    }

    pub fn note_to_int(note: &str) -> i32 {
        let note_dict = hashmap!['C' => 0, 'D' => 2, 'E' => 4, 'F' => 5, 'G' => 7, 'A' => 9, 'B' => 11];
        

        // TODO: check if note is valid before getting root
        // first char of note
        let root_note = &note.chars().next().unwrap();
        // convert root to int
        let mut val = note_dict[root_note];

        // iterate over any accidentals
        for c in note.chars() {
            if c == 'b' {
                if val == 0 {
                    val = 11
                } else {
                    val -= 1;
                }
            } else if c == '#' {
                val += 1;
            }
        }

        // return final int
        val % 12
    }
}

#[allow(dead_code)]
mod intervals {
    use super::notes;

    pub fn third(note: &str) -> String {
        return interval(note, 2)
    }

    pub fn fifth(note: &str) -> String {
        return interval(note, 4)
    }

    pub fn major_third(note: &str) -> String {
        let note_root = note.chars().next().unwrap().to_string();
        let third = third(&note_root);
        return augment_or_diminish_until(note, &third, 4);
    }

    pub fn minor_third(note: &str) -> String {
        let note_root = note.chars().next().unwrap().to_string();
        let third = third(&note_root);
        return augment_or_diminish_until(note, &third, 3);
    }

    pub fn major_fifth(note: &str) -> String {
        let note_root = note.chars().next().unwrap().to_string();
        let fifth = fifth(&note_root);
        return augment_or_diminish_until(note, &fifth, 7);
    }

    pub fn minor_fifth(note: &str) -> String {
        let note_root = note.chars().next().unwrap().to_string();
        let fifth = fifth(&note_root);
        return augment_or_diminish_until(note, &fifth, 6);
    }

    pub fn perfect_fifth(note: &str) -> String {
        return major_fifth(note)
    }

    pub fn diminish_until(note1: &str, note2: &str, interval: usize, mut current: usize) -> usize {
        let note2 = &notes::diminish(note2);
        current = measure(note1, note2);
        if current > interval {
            current = diminish_until(note1, note2, interval, current);
        }
        return current
    }

    pub fn augment_until(note1: &str, note2: &str, interval: usize, mut current: usize) -> usize {
        let note2 = &notes::augment(note2);
        current = measure(note1, note2);
        if current < interval {
            current = augment_until(note1, note2, interval, current);
        }
        return current
    }

    pub fn augment_or_diminish_until(note1: &str, note2: &str, interval: usize) -> String {
        let diff = measure(note1, note2);
        let mut result = note2.to_string();

        // TODO: unless diminish_until and augment_until are solving some corner case
        //       they can probably be simplified down to getting the magnitude between 
        //       measure and the input interval
        if diff > interval {
            let current = diminish_until(note1, note2, interval, diff);
            result = result + &"b".repeat(diff - current);
        } else if diff < interval {
            let current = augment_until(note1, note2, interval, diff);
            result = result + &"#".repeat(current - diff);
        }

        let mut num_accidentals = 0;
        for (i, accidental) in note2.chars().enumerate() {
            // skip note root
            if i == 0 { continue }

            if accidental == '#' {
                num_accidentals += 1;
            } else if accidental == 'b' {
                num_accidentals -= 1;
            }
        }

        if num_accidentals > 6 {
            num_accidentals = -12 + (num_accidentals % 12);
        } else if num_accidentals < -6 {
            num_accidentals = 12 + (num_accidentals % 12)
        }

        while num_accidentals > 0  {
            result = notes::augment(&result);
            num_accidentals -= 1;
        }
        while num_accidentals < 0 {
            result = notes::diminish(&result);
            num_accidentals += 1;
        }

        return result
    }

    pub fn interval(start_note: &str, interval: usize) -> String {
        // hardcoded as key of C right now. I belive this should still work for 
        // recognizing all chords which is the only functionality I need. 
        let notes_in_key = ["C", "D", "E", "F", "G", "A", "B"];
        let ascii_start_note_root = start_note.bytes().next().unwrap();
        let mut index: usize = 0;

        for (i, note) in notes_in_key.iter().enumerate() {
            let ascii_note_in_key = note.bytes().next().unwrap();
            if  ascii_note_in_key == ascii_start_note_root {
                index = (i + interval) % 7;
                break;
            }
        }

        return notes_in_key[index].to_string()
    }

    // returns the number of half steps between two notes
    pub fn measure(note1: &str, note2: &str) -> usize {
        // get int val of each note
        let note1_int = notes::note_to_int(note1);
        let note2_int = notes::note_to_int(note2);
        // subtract to get difference
        let diff = note2_int - note1_int;
        
        // return normalized number
        let result: i32;
        if diff < 0 {
            result = 12 - diff * -1;
        } else {
            result = diff;
        }

        return result as usize
    }

    pub fn determine(note1: &str, note2: &str) -> String {
        let note1_root = note1.chars().next().unwrap();
        let note2_root = note2.chars().next().unwrap();

        // Corner case for unisons        
        if note1_root == note2_root {
            // count values of accidentals
            fn get_val(note: &str) -> i32 {
                let mut val = 0;
                for c in note.chars() {
                    if c == 'b' {
                        val -= 1;
                    } else if c == '#' {
                        val += 1;
                    }
                }
                val 
            }

            let val1 = get_val(note1);
            let val2 = get_val(note2);

            // TODO: Add shorthand conditional and results
            if val1 == val2 {
                return String::from("1")
            } else if val1 < val2 {
                return String::from("#1")
            } else if val1 > val2 {
                return String::from("b1")
            } else {
                return String::from("bb1")
            }
        }
        
        // Other Intervals
        let n1 = notes::FIFTHS.iter().position(|&s| s == note1_root).unwrap();
        let n2 = notes::FIFTHS.iter().position(|&s| s == note2_root).unwrap();

        let number_of_fifth_steps: usize;
        if n2 < n1 {
            number_of_fifth_steps = notes::FIFTHS.len() + n2 - n1;
        } else {
            number_of_fifth_steps = n2 - n1;
        }
        let fifth_steps = [("unison",  "1", 0),
                           ("fifth",   "5", 7),
                           ("second",  "2", 2),
                           ("sixth",   "6", 9),
                           ("third",   "3", 4),
                           ("seventh", "7", 11),
                           ("fourth",  "4", 5)];

        // count half steps between note1 and note2
        let half_notes = measure(note1, note2);

        // get tuple for cooresponding number of fifths between our two notes
        let num_fifths = fifth_steps[number_of_fifth_steps];

        // number of major steps for this interval
        let major = num_fifths.2;

        // if major steps is equal to half steps the interval is major or perfect
        if major == half_notes {
            // Perfect
            return num_fifths.1.to_string()
        } else if major + 1 <= half_notes {
            // Augmented
            let num_sharps = half_notes - major;
            return "#".repeat(num_sharps) + &num_fifths.1
        } else if major - 1 == half_notes {
            // Minor
            return "b".to_string() + &num_fifths.1
        } else {
            // Diminished
            let num_flats = major - half_notes;
            return "b".repeat(num_flats) + &num_fifths.1
        }
    }
}

pub mod chords {
    use super::intervals;
    use super::notes;

    pub fn major_triad(root: &str) -> Vec<String> {
        let second_note = intervals::major_third(root);
        let third_note = intervals::perfect_fifth(root);
        return vec![root.to_string(), second_note, third_note]
    }

    pub fn minor_triad(root: &str) -> Vec<String> {
        let second_note = intervals::minor_third(root);
        let third_note = intervals::perfect_fifth(root);
        return vec![root.to_string(), second_note, third_note]
    }

    pub fn diminished_triad(root: &str) -> Vec<String> {
        let second_note = intervals::minor_third(root);
        let third_note = intervals::minor_fifth(root);
        return vec![root.to_string(), second_note, third_note]
    }

    pub fn augmented_triad(root: &str) -> Vec<String> {
        let second_note = intervals::major_third(root);
        let third_note = notes::augment(&intervals::major_fifth(root));
        return vec![root.to_string(), second_note, third_note]
    }

    pub fn determine_triad(triad: Vec<&str>) -> Vec<String> {
        // initialize result
        let chords: Vec<(String, i32, String)> = Vec::new();

        fn inversion_exhauster(triad: Vec<&str>, tries: i32, mut chords: Vec<(String, i32, String)>) -> Vec<String> {

            // get intervals between 1&2 and 1&3 keys
            let interval1 = intervals::determine(&triad[0], &triad[1]);
            let interval2 = intervals::determine(&triad[0], &triad[2]);
            let intervals = interval1 + &interval2;


            match &*intervals {
                "25" => chords.push(("sus2".to_string(), tries, triad[0].to_string())),
                "3b7" => chords.push(("dom7".to_string(), tries, triad[0].to_string())),
                "3b5" => chords.push(("7b5".to_string(), tries, triad[0].to_string())),
                "35" => chords.push(("M".to_string(), tries, triad[0].to_string())),
                "3#5" => chords.push(("Augmented Triad".to_string(), tries, triad[0].to_string())),
                "36" => chords.push(("M6".to_string(), tries, triad[0].to_string())),
                "37" => chords.push(("M7".to_string(), tries, triad[0].to_string())),
                "b3b5" => chords.push(("Diminished Triad".to_string(), tries, triad[0].to_string())),
                "b35" => chords.push(("m".to_string(), tries, triad[0].to_string())),
                "b36" => chords.push(("m6".to_string(), tries, triad[0].to_string())),
                "b3b7" => chords.push(("m7".to_string(), tries, triad[0].to_string())),
                "b37" => chords.push(("m/M7".to_string(), tries, triad[0].to_string())),
                "45" => chords.push(("sus4".to_string(), tries, triad[0].to_string())),
                "5b7" => chords.push(("m7".to_string(), tries, triad[0].to_string())),
                "57" => chords.push(("M7".to_string(), tries, triad[0].to_string())),
                _ => (),
            }

            let mut results = Vec::new();

            if tries != 3 {
                let invert_chord = vec![triad[2], triad[0], triad[1]];
                return inversion_exhauster(invert_chord, tries + 1, chords)
            } else {
                for chord in chords.iter() {
                    results.push(chord.2.clone() + &chord.0)
                }
            }

            return results
        }

        return inversion_exhauster(triad, 1, chords)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_<function_name>() {
    //     assert_eq!(module::function(), answer);
    // }

    #[test]
    fn test_determine_triad() {
        assert_eq!(determine_triad("C,E,G"), "C Major Triad");
    }

    #[test]
    fn test_chords_major_triad() {
        assert_eq!(chords::major_triad("C"), vec!["C", "E", "G"]);
        assert_eq!(chords::major_triad("F#"), vec!["F#", "A#", "C#"]);
        assert_eq!(chords::major_triad("Db"), vec!["Db", "F", "Ab"]);
    }

    #[test]
    fn test_chords_minor_triad() {
        assert_eq!(chords::minor_triad("C"), vec!["C", "Eb", "G"]);
        assert_eq!(chords::minor_triad("F#"), vec!["F#", "A", "C#"]);
        assert_eq!(chords::minor_triad("Db"), vec!["Db", "Fb", "Ab"]);
    }

    #[test]
    fn test_chords_diminished_triad() {
        assert_eq!(chords::diminished_triad("C"), vec!["C", "Eb", "Gb"]);
        assert_eq!(chords::diminished_triad("F#"), vec!["F#", "A", "C"]);
        assert_eq!(chords::diminished_triad("Db"), vec!["Db", "Fb", "Abb"]);
    }

    #[test]
    fn test_chords_augmented_triad() {
        assert_eq!(chords::augmented_triad("C"), vec!["C", "E", "G#"]);
        assert_eq!(chords::augmented_triad("F#"), vec!["F#", "A#", "C##"]);
        assert_eq!(chords::augmented_triad("Db"), vec!["Db", "F", "A"]);
    }

    #[test]
    fn test_intervals_major_third() {
        assert_eq!(intervals::major_third("C"), "E");
        assert_eq!(intervals::major_third("C#"), "E#");
        assert_eq!(intervals::major_third("Cb"), "Eb");
    }

    #[test]
    fn test_intervals_major_fifth() {
        assert_eq!(intervals::major_fifth("C"), "G");
        assert_eq!(intervals::major_fifth("C#"), "G#");
        assert_eq!(intervals::major_fifth("Cb"), "Gb");
    }

    #[test]
    fn test_augment_or_diminish_until() {
        assert_eq!(intervals::augment_or_diminish_until("C", "D", 8), "D######");
        assert_eq!(intervals::augment_or_diminish_until("D", "C", 8), "Cbb");
    }

    #[test]
    fn test_notes_augment() {
        assert_eq!(notes::augment("A"), "A#");
        assert_eq!(notes::augment("Ab"), "A");
        assert_eq!(notes::augment("A#"), "A##");
    }

    #[test]
    fn test_notes_diminish() {
        assert_eq!(notes::diminish("A"), "Ab");
        assert_eq!(notes::diminish("Ab"), "Abb");
        assert_eq!(notes::diminish("A#"), "A");
    }

    #[test]
    fn test_intervals_interval() {
        assert_eq!(intervals::interval("C", 2), "E");
        assert_eq!(intervals::interval("Cb", 2), "E");
        assert_eq!(intervals::interval("C#", 2), "E");
        assert_eq!(intervals::interval("G", 3), "C");
    }

    #[test]
    fn test_chords_determine() {
        assert_eq!(chords::determine_triad(vec!["C", "E", "G"]), vec!["C Major Triad"]);
        assert_eq!(chords::determine_triad(vec!["C", "Eb", "G"]), vec!["C Minor Triad", "Eb M6"]);
        assert_eq!(chords::determine_triad(vec!["C", "Eb", "Gb"]), vec!["C Diminished Triad", "Eb m6"]);
        assert_eq!(chords::determine_triad(vec!["C", "E", "G#"]), vec!["C Augmented Triad"]);
    }

    #[test]
    fn test_note_to_int_conversion() {
        assert_eq!(notes::note_to_int("Cb"), 11);        
        assert_eq!(notes::note_to_int("C"), 0);
        assert_eq!(notes::note_to_int("C#"), 1);
        assert_eq!(notes::note_to_int("Db"), 1);        
        assert_eq!(notes::note_to_int("D"), 2);
        assert_eq!(notes::note_to_int("D#"), 3);
        assert_eq!(notes::note_to_int("Eb"), 3);
        assert_eq!(notes::note_to_int("E"), 4);
        assert_eq!(notes::note_to_int("E#"), 5);
        assert_eq!(notes::note_to_int("Fb"), 4);                
        assert_eq!(notes::note_to_int("F"), 5);
        assert_eq!(notes::note_to_int("F#"), 6);
        assert_eq!(notes::note_to_int("Gb"), 6);        
        assert_eq!(notes::note_to_int("G"), 7);
        assert_eq!(notes::note_to_int("G#"), 8);
        assert_eq!(notes::note_to_int("Ab"), 8);        
        assert_eq!(notes::note_to_int("A"), 9);
        assert_eq!(notes::note_to_int("A#"), 10);
        assert_eq!(notes::note_to_int("Bb"), 10);        
        assert_eq!(notes::note_to_int("B"), 11);
        assert_eq!(notes::note_to_int("B#"), 0);        
    }

    #[test]
    fn test_interval_measure() {
        assert_eq!(intervals::measure("C", "D"), 2);
        assert_eq!(intervals::measure("B", "C"), 1);
        assert_eq!(intervals::measure("C", "B"), 11);
        assert_eq!(intervals::measure("F", "F#"), 1);
        assert_eq!(intervals::measure("Ab", "A"), 1);
        assert_eq!(intervals::measure("C", "C"), 0);
    }

    #[test]
    fn test_interval_determine() {
        assert_eq!(intervals::determine("G", "C"), "4");
        assert_eq!(intervals::determine("C", "C"), "1");
        assert_eq!(intervals::determine("C", "C#"), "#1");
        assert_eq!(intervals::determine("C", "D"), "2");
        assert_eq!(intervals::determine("C", "D#"), "#2");
        assert_eq!(intervals::determine("C", "E"), "3");
        assert_eq!(intervals::determine("C", "E#"), "#3");
        assert_eq!(intervals::determine("C", "F"), "4");
        assert_eq!(intervals::determine("C", "F#"), "#4");
        assert_eq!(intervals::determine("C", "G"), "5");
        assert_eq!(intervals::determine("C", "G#"), "#5");
        assert_eq!(intervals::determine("C", "A"), "6");
        assert_eq!(intervals::determine("C", "A#"), "#6");
        assert_eq!(intervals::determine("C", "B"), "7");
        // assert_eq!(intervals::determine("C", "B#"), "bbbbbbbbbbb7"); // I think this is a bug
    }
}