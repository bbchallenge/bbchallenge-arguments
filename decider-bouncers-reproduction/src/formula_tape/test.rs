use std::collections::HashSet;

use super::*;
#[test]
/// Proving that https://bbchallenge.org/43477769 is a bouncer (i.e. non-halting) from a given formula tape (i.e. formula tape not guessed).
fn no_guessing_prove_bouncer_43_477_769() {
    let machine_str = "1RB0RD_1LC1LE_1RA1LB_---0RC_1LB0LE";
    let formula_tape_str =
        "0∞<E000011110(11110111101111011110)000(1111011110)000(11110)000(11110)011111110∞";
    let mut formula_tape = FormulaTape::from_str(formula_tape_str).unwrap();
    formula_tape.set_machine_str(machine_str);
    assert_eq!(format!("{formula_tape}"), formula_tape_str);

    let cert = formula_tape.prove_non_halt(200_000, 0).unwrap().unwrap();
    assert_eq!(cert.num_macro_steps_until_special_case, 1118);
}

#[test]
/// Proving that https://bbchallenge.org/88427177 is a bouncer (i.e. non-halting) from a given formula tape (i.e. formula tape not guessed).
fn no_guessing_prove_bouncer_88_427_177() {
    let machine_str = "1RB1LE_1LC1RD_1LB1RC_1LA0RD_---0LA";
    let formula_tape_str = "0∞(111)1110(11)00D>0∞";
    let mut formula_tape = FormulaTape::from_str(formula_tape_str).unwrap();
    formula_tape.set_machine_str(machine_str);
    assert_eq!(format!("{formula_tape}"), formula_tape_str);

    let cert = formula_tape.prove_non_halt(200_000, 0).unwrap().unwrap();
    assert_eq!(cert.num_macro_steps_until_special_case, 41);
}

#[test]
/// Proving that https://bbchallenge.org/6416853 is a bouncer (i.e. non-halting) from a given formula tape (i.e. formula tape not guessed).
fn no_guessing_prove_bouncer_6_416_853() {
    let machine_str = "1RB0LC_0LA1RC_0LD0LE_1LA1RA_---1LC";
    let formula_tape_str = "0∞<C(10)00(0)0∞";
    let mut formula_tape = FormulaTape::from_str(formula_tape_str).unwrap();
    formula_tape.set_machine_str(machine_str);
    assert_eq!(format!("{formula_tape}"), formula_tape_str);

    let cert = formula_tape.prove_non_halt(200_000, 0).unwrap().unwrap();
    assert_eq!(cert.num_macro_steps_until_special_case, 13);
}

#[test]
fn decide_bouncer_43_477_769() {
    use super::bouncers_decider::bouncers_decider;
    let machine_str = "1RB0RD_1LC1LE_1RA1LB_---0RC_1LB0LE";
    let cert: BouncerCertificate = bouncers_decider(machine_str, 6000, 2000, 10)
        .unwrap()
        .unwrap();
    println!("Formula tape:\n{}", cert.formula_tape);
    assert_eq!(cert.num_steps_until_formula_tape, 1365);
    assert_eq!(cert.num_macro_steps_until_special_case, 1892);
}

#[test]
fn decide_bouncer_88_427_177() {
    use super::bouncers_decider::bouncers_decider;
    let machine_str = "1RB1LE_1LC1RD_1LB1RC_1LA0RD_---0LA";
    let cert: BouncerCertificate = bouncers_decider(machine_str, 200, 2000, 10)
        .unwrap()
        .unwrap();
    println!("Formula tape:\n{}", cert.formula_tape);
    assert_eq!(cert.num_steps_until_formula_tape, 76);
    assert_eq!(cert.num_macro_steps_until_special_case, 47);
}

#[test]
fn decide_bouncer_6_416_853() {
    use super::bouncers_decider::bouncers_decider;
    let machine_str = "1RB0LC_0LA1RC_0LD0LE_1LA1RA_---1LC";
    let cert = bouncers_decider(machine_str, 1000, 2000, 10)
        .unwrap()
        .unwrap();

    println!("Formula tape: {}", cert.formula_tape);

    assert_eq!(cert.num_steps_until_formula_tape, 705);
    assert_eq!(cert.num_macro_steps_until_special_case, 97);
}

#[test]
fn decider_bouncer_892_918() {
    use super::bouncers_decider::bouncers_decider;
    let machine_str = "1RB---_0LC0RB_1RA1LD_1LE1LD_1LB1LC";
    let cert = bouncers_decider(machine_str, 10000, 10000, 10)
        .unwrap()
        .unwrap();

    println!("Formula tape: {}", cert.formula_tape);

    assert_eq!(cert.num_steps_until_formula_tape, 4835);
    assert_eq!(cert.num_macro_steps_until_special_case, 2134);
}

#[test]
fn decider_bouncer_13_138_739() {
    use super::bouncers_decider::bouncers_decider;
    let machine_str = "1RB1LD_1RC0RC_1RD1RA_1LE1LA_---0LA";
    let cert = bouncers_decider(machine_str, 10000, 10000, 10)
        .unwrap()
        .unwrap();

    println!("Formula tape: {}", cert.formula_tape);

    assert_eq!(cert.num_steps_until_formula_tape, 705);
    assert_eq!(cert.num_macro_steps_until_special_case, 97);
}

#[test]
fn decider_bouncer_83_795_500() {
    // This bouncer encounters a looper shift rule on
    // 0∞110011001100A>(11000100)01000100010∞
    use super::bouncers_decider::bouncers_decider;
    let machine_str = "1RB1LD_1RC1RE_1LA0LC_0RA0LA_0RD---";
    let cert = bouncers_decider(machine_str, 200, 10000, 10)
        .unwrap()
        .unwrap();

    println!("Formula tape: {}", cert.formula_tape);

    assert_eq!(cert.num_steps_until_formula_tape, 705);
    assert_eq!(cert.num_macro_steps_until_special_case, 97);
}

// Testing formula tape iterator
#[test]
fn iterate_formula_tapes_bouncer_88_427_177() -> Result<(), FormulaTapeError> {
    use super::bouncers_decider::bouncers_decider;
    use super::formula_tape_guessing::{
        guess_formula_tape_given_record_breaking_tapes,
        guess_formula_tapes_given_record_breaking_tapes, PotentialFormulaTape,
    };
    use itertools::Itertools;
    use std::collections::HashMap;
    // Machine "1RB---_0RC0RD_1LD1RE_0LE---_1RB0LB" is not solved with the first formula tape guessed.
    let machine_str = "1RB---_0RC0RD_1LD1RE_0LE---_1RB0LB";
    let mut record_breaking_tapes: HashMap<TapeHead, Vec<Tape>> = HashMap::new();
    let mut tape = Tape::new_initial(machine_str);
    record_breaking_tapes.insert(tape.get_current_head()?, vec![tape.clone()]);

    let step_limit = 10000;
    for _ in 0..step_limit {
        tape.step()?;

        if tape.get_current_read_pos()? == 0 || tape.get_current_read_pos()? == tape.len() - 1 {
            match record_breaking_tapes.get_mut(&tape.get_current_head()?) {
                Some(tapes) => {
                    tapes.push(tape.clone());
                }
                None => {
                    record_breaking_tapes.insert(tape.get_current_head()?, vec![tape.clone()]);
                }
            }
        }
    }

    let mut another_found = false;
    for head in record_breaking_tapes.keys().sorted() {
        let tapes = record_breaking_tapes.get(head).unwrap();
        println!("HEAD {}", head);
        let res = guess_formula_tapes_given_record_breaking_tapes(&tapes);

        if res.is_empty() {
            continue;
        }

        println!("Ground truth formula tapes");
        for (formula_tape, steps) in res.iter() {
            println!("{} {}", formula_tape, steps);
        }

        let mut potential_formula = PotentialFormulaTape {
            record_breaking_tapes: tapes.clone(),
            tested_tape_length: HashSet::new(),
            current_index_in_tapes_length: 0,
            tested_len1_len2_pairs: HashSet::new(),
            tested_len_diff_and_step_diff2: Vec::new(),
        };
        let (formula_tape, steps) = potential_formula.next().unwrap();
        println!("{} {}", formula_tape, steps);
        println!(
            "{:?} {} {:?}",
            potential_formula.tested_tape_length,
            potential_formula.current_index_in_tapes_length,
            potential_formula.tested_len1_len2_pairs
        );
        if !potential_formula.next().is_none() {
            another_found = true;
            println!("{} {}", formula_tape, steps);
            println!(
                "{:?} {} {:?}",
                potential_formula.tested_tape_length,
                potential_formula.current_index_in_tapes_length,
                potential_formula.tested_len1_len2_pairs
            );
        }

        break;
    }
    assert!(another_found);
    Ok(())
}
