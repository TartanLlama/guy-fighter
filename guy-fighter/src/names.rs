//https://www.imagineforest.com/blog/funny-name-generator/

pub const FIRST_NAMES: [&str; 50] = [
    "Terge", "Zoowee", "Fluffy", "Buritt", "Peaberry",
    "Trashwee", "Flapberry", "Gummoo", "Humster", "Burberry",
    "Slugtu", "Bofy", "Sniffpants", "Stinkmoo", "Barfberry",
    "Barfaboo", "Wormbag", "Peaster", "Figitt", "Subo",
    "Pea-a-boo", "Zoobuns", "Foobug", "Peamoo", "Ratbuns",
    "Madaloo", "Peabs", "Bushbo", "Binfy", "Bittyspitz",
    "Fartmoo", "Binpants", "Chewspitz", "Gootu", "Bobuns",
    "Pooritt", "Stinkwee", "Chewitt", "Snortu", "Fartaloo",
    "Bugbee", "Poopfy", "Zoozy", "Trashbug", "Peaman",
    "Stinkbuns", "Bittypants", "Bugeenie", "Dingbs", "Snabster"
];

pub const LAST_NAMES: [&str; 50] = [
    "Sternherd", "Blubberworth", "Gloomkins", "Noseface", "Wigglewhistle",
    "Sockborn", "Fudgewhistle", "Hooperbottom", "Pottyworthy", "Gloomhall",
    "Hooperseed", "Gloomman", "Pimplehill", "Snotsniff", "Oinkhill",
    "Roachson", "Chewface", "Beeman", "Tootson", "Doozyton",
    "Moanihill", "Sockball", "Swamprider", "Fudgehill", "Wigglefish",
    "Snotseed", "Blubberseed", "Tootbean", "Moanigold", "Boogerfeet",
    "PimpleFadden", "Noodlefeet", "Snotborn", "Messyhall", "Pimplehill",
    "Woofhair", "Goatbottom", "Pottybag", "Madborn", "Roachhall",
    "Noseson", "Chewbrain", "Beaniehall", "Messyhall", "Oinkson",
    "Spottygold", "Messyrider", "Roachson", "Moonbean", "Toothill"
];

pub fn generate_name() -> String {
    let first_name = FIRST_NAMES[rand::random::<usize>() % FIRST_NAMES.len()];
    let last_name = LAST_NAMES[rand::random::<usize>() % LAST_NAMES.len()];
    format!("{} {}", first_name, last_name)
}