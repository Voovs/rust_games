#[allow(dead_code)]
mod party_events {
    extern crate rand;
    // use std::io;
    use rand::{ thread_rng, Rng };


    enum InterruptionAtParty {
        GlassShatter,
        PedroWalksIn,
        TechnicalDifficulties,
        SpillPunch,
        Heist(u8),
        BrennanaPeel{ x: u8, y: u8 },
    }


    type PartyInter = InterruptionAtParty;


    pub fn main() {
        type Pi = PartyInter;

        if let Some(p) = new_party_event() {
            match p {
                Pi::GlassShatter => { println!("Was it surprise or carelessness that turned the floor sinister with glistening sharpness? We don't know and won't stay long enough to find out") },
                Pi::PedroWalksIn => { println!("It becomes hard to breath. A heavy white fog rolls out from the doorway. Is it just me or is it getting hot in here? Pedro has just walked in") },
                Pi::TechnicalDifficulties => { println!("The lights suddenly blow out. By the time they come back on, everyone is ready to leave") },
                Pi::SpillPunch => { println!("Carrying a bowl of punch, ") },
                Pi::Heist(time) => { println!("At {}h, on the dot, a heist party takes the party hostage. Maybe next time don't hold parties right below a bank", time) },
                Pi::BrennanaPeel{ x: w, y: e } => { println!("There was a Brennana peel on the floor {}m from the door and {}m down the corridor", w, e) },
            }
        } else {
            println!("Disaster has been avoided. The party goes as expected");
        }
    }


    fn new_party_event() -> Option<PartyInter> {
        let mut rng = thread_rng();

        let event_num = rng.gen_range(1, 11);
        let modifier_num = rng.gen_range(1, 25);

        let event: Option<PartyInter> = match event_num {
            6 => Some(PartyInter::GlassShatter),
            7 => Some(PartyInter::TechnicalDifficulties),
            8 => Some(PartyInter::SpillPunch),
            9 => Some(PartyInter::Heist(modifier_num)),
            10 => Some(PartyInter::BrennanaPeel{x: modifier_num * modifier_num, y: modifier_num * event_num / 4}),
            _ => None,
        };

        event
    }
}

#[allow(unused_imports)]
#[allow(unused_mut)]
mod games;

#[allow(unreachable_code)]
mod menu;

#[cfg(test)]
mod tests;


fn main() {
    loop {
        menu::select_game();
        if !menu::is_play_again() { break; }
    }
}
