use std::sync::Mutex;

use rand::prelude::*;

lazy_static! {
    pub static ref QUESTION_ANSWERS: Mutex<Vec<(&'static str, &'static str)>> = {
        let mut qa = Vec::new();

        qa.push((
            "Name of the class (type) which provides access to a single setting inside of a BepInEx ConfigFile.",
            "ConfigEntry"
    ));

        // qa.push((
        //     "Name of the class (type) which provides access to the various properties .",
        //     "CharacterBody",
        // ));

        Mutex::new(qa)
    };
}

pub(crate) fn give_random_dev_question_answer() -> Vec<(&'static str, &'static str)> {
    let mut rng = &mut rand::thread_rng();

    let qa = QUESTION_ANSWERS.lock().unwrap();

    let x = qa.choose_multiple(&mut rng, 1).cloned();

    x.collect()
}
