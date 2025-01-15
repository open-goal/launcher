use std::vec;

pub struct MilestoneCriteria {
  pub name: String,
  // some milestones are considered reached once you've completed something
  // (ie. collecting a cell in an area)
  pub completed: Vec<u8>,
  // others are reached when they've been introduced
  // (ie. preparing for a boss fight)
  pub introduced: Vec<u8>,
}

pub struct GameTaskStatus {
  pub introduced: bool,
  pub completed: bool,
}

pub fn get_jak1_milestones() -> Vec<MilestoneCriteria> {
  vec![
    MilestoneCriteria {
      name: "geyser".to_string(),
      completed: vec![],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (village1-yakow 10)
      // (village1-mayor-money 11)
      // (village1-uncle-money 12)
      // (village1-oracle-money1 13)
      // (village1-oracle-money2 14)
      // (beach-ecorocks 15)
      // (village1-buzzer 75)
      name: "sandover".to_string(),
      completed: vec![10, 11, 12, 13, 14, 75],
      introduced: vec![15],
    },
    MilestoneCriteria {
      // (beach-ecorocks 15)
      // (beach-pelican 16)
      // (beach-flutflut 17)
      // (beach-seagull 18)
      // (beach-cannon 19)
      // (beach-buzzer 20)
      // (beach-gimmie 21)
      // (beach-sentinel 22)
      name: "sentinel".to_string(),
      completed: vec![15, 16, 17, 18, 19, 20, 21, 22],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (jungle-eggtop 2)
      // (jungle-lurkerm 3)
      // (jungle-tower 4)
      // (jungle-fishgame 5)
      // (jungle-plant 6)
      // (jungle-buzzer 7)
      // (jungle-canyon-end 8)
      // (jungle-temple-door 9)
      name: "jungle".to_string(),
      completed: vec![2, 3, 4, 5, 6, 7, 8, 9],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (misty-muse 23)
      // (misty-boat 24)
      // (misty-warehouse 25)
      // (misty-cannon 26)
      // (misty-bike 27)
      // (misty-buzzer 28)
      // (misty-bike-jump 29)
      // (misty-eco-challenge 30)
      // (leaving-misty 114)
      name: "misty".to_string(),
      completed: vec![23, 24, 25, 26, 27, 28, 29, 30, 114],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (firecanyon-buzzer 68)
      // (firecanyon-end 69)
      // (firecanyon-assistant 102)
      name: "firecanyon".to_string(),
      completed: vec![68, 69],
      introduced: vec![102],
    },
    MilestoneCriteria {
      // (village2-gambler-money 31)
      // (village2-geologist-money 32)
      // (village2-warrior-money 33)
      // (village2-oracle-money1 34)
      // (village2-oracle-money2 35)
      // (firecanyon-buzzer 68)
      // (firecanyon-end 69)
      // (village2-buzzer 76)
      // (firecanyon-assistant 102)
      name: "village2".to_string(),
      completed: vec![31, 32, 33, 34, 35, 68, 69],
      introduced: vec![76, 102],
    },
    MilestoneCriteria {
      // (rolling-race 52)
      // (rolling-robbers 53)
      // (rolling-moles 54)
      // (rolling-plants 55)
      // (rolling-lake 56)
      // (rolling-buzzer 57)
      // (rolling-ring-chase-1 58)
      // (rolling-ring-chase-2 59)
      name: "basin".to_string(),
      completed: vec![52, 53, 54, 55, 56, 57, 58, 59],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (swamp-billy 36)
      // (swamp-flutflut 37)
      // (swamp-battle 38)
      // (swamp-tether-1 39)
      // (swamp-tether-2 40)
      // (swamp-tether-3 41)
      // (swamp-tether-4 42)
      // (swamp-buzzer 43)
      // (swamp-arm 104)
      name: "swamp".to_string(),
      completed: vec![36, 37, 38, 39, 40, 41, 42, 43, 104],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (sunken-platforms 44)
      // (sunken-pipe 45)
      // (sunken-slide 46)
      // (sunken-room 47)
      // (sunken-sharks 48)
      // (sunken-buzzer 49)
      // (sunken-top-of-helix 50)
      // (sunken-spinning-room 51)
      name: "lpc".to_string(),
      completed: vec![44, 45, 46, 47, 48, 49, 50, 51],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (ogre-boss 86)
      // (village2-levitator 103)
      name: "klaww".to_string(),
      completed: vec![103],
      introduced: vec![86],
    },
    MilestoneCriteria {
      // (ogre-boss 86)
      // (ogre-end 87)
      // (ogre-buzzer 88)
      // (ogre-secret 110)
      name: "mountainpass".to_string(),
      completed: vec![86, 88, 110],
      introduced: vec![87],
    },
    MilestoneCriteria {
      // (village3-extra1 74)
      // (village3-buzzer 77)
      // (village3-miner-money1 96)
      // (village3-miner-money2 97)
      // (village3-miner-money3 98)
      // (village3-miner-money4 99)
      // (village3-oracle-money1 100)
      // (village3-oracle-money2 101)
      // (village3-button 105)
      name: "village3".to_string(),
      completed: vec![74, 77, 96, 97, 98, 99, 100, 101, 105],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (cave-gnawers 78)
      // (cave-dark-crystals 79)
      // (cave-dark-climb 80)
      // (cave-robot-climb 81)
      // (cave-swing-poles 82)
      // (cave-spider-tunnel 83)
      // (cave-platforms 84)
      // (cave-buzzer 85)
      name: "cave".to_string(),
      completed: vec![78, 79, 80, 81, 82, 83, 84, 85],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (snow-eggtop 60)
      // (snow-ram 61)
      // (snow-fort 62)
      // (snow-ball 63)
      // (snow-bunnies 64)
      // (snow-buzzer 65)
      // (snow-bumpers 66)
      // (snow-cage 67)
      name: "snowy".to_string(),
      completed: vec![60, 61, 62, 63, 64, 65, 66, 67],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (lavatube-end 89)
      // (lavatube-buzzer 90)
      // (lavatube-balls 107)
      // (lavatube-start 108)
      // (assistant-village3 115)
      name: "lavatube".to_string(),
      completed: vec![90, 107, 108, 115],
      introduced: vec![89],
    },
    MilestoneCriteria {
      // (citadel-sage-green 70)
      // (citadel-sage-blue 71)
      // (citadel-sage-red 72)
      // (citadel-sage-yellow 73)
      // (lavatube-end 89)
      // (citadel-buzzer 91)
      name: "citadel".to_string(),
      completed: vec![71, 72, 73, 89, 91],
      introduced: vec![70],
    },
    MilestoneCriteria {
      // (citadel-sage-green 70)
      name: "finalboss".to_string(),
      completed: vec![70],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (finalboss-movies 112)
      name: "end".to_string(),
      completed: vec![],
      introduced: vec![112],
    },
  ]
}
