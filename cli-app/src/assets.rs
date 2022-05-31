pub(crate) fn questions() -> Vec<String> {
    vec![
        "Who is the smartest person alive?",
        "I only believe in __.",
        "_ and _ are the best things in the world.",
        "I take pride in my _, __, and ___.",
        "_,_,_.",
    ]
    .into_iter()
    .map(|p| p.to_owned())
    .collect()
}

pub(crate) fn answers() -> Vec<String> {
    vec![
        "Zombie Apocalypse",
        "Winne the Pooh",
        "video",
        "blood",
        "cute girl",
        "shopping",
        "loud speaker",
        "response",
        "smart method",
        "instance of the class",
        "population",
        "pollution",
        "smelly garbage",
        "argument",
        "tennis ball",
        "hearing",
        "estate",
        "refrigerator",
        "transportation",
        "food",
        "piano",
        "president",
        "fact",
        "poet",
        "permission",
        "ear",
        "scary ugly insect",
        "indication",
        "bad medicine",
        "history",
        "election",
        "requirement",
        "establishment",
        "agreement",
        "grocery",
        "big camera",
        "reputation",
        "solution",
        "role",
        "scary long statement",
        "impression",
        "interaction",
        "winner",
        "photo",
        "republic",
        "yummy pizza",
        "grey computer",
        "profession",
        "paid membership",
        "university",
        "high income",
        "big data",
    ]
    .into_iter()
    .map(|p| p.to_owned())
    .collect()
}
