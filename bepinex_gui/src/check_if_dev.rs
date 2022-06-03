use std::sync::Mutex;

lazy_static! {
    pub static ref QUESTIONS_ANSWERS: Mutex<Vec<(&'static str, &'static str)>> = {
        let mut qa = Vec::new();

        qa.push((
            "Give the name of the class (type) which provides access to a single setting inside of a BepInEx ConfigFile.",
            "ConfigEntry"
    ));

        qa.push((
            "Give the name of the class (type) which provides access to the various properties like damage or crit, of a survivor in Risk of Rain 2.",
            "CharacterBody",
        ));

        Mutex::new(qa)
    };
    pub static ref QUESTION_ANSWERS_LENGTH: usize = {
        let qa = QUESTIONS_ANSWERS.lock().unwrap();
        qa.len()
    };
}

// pub(crate) fn give_random_dev_question_answer() -> Vec<(&'static str, &'static str)> {
//     let mut rng = &mut rand::thread_rng();

//     let qa = QUESTION_ANSWERS.lock().unwrap();

//     let x = qa.choose_multiple(&mut rng, 1).cloned();

//     x.collect()
// }
