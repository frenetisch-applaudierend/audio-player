use crate::player::Player;

pub fn draw_frame(player: &mut Player) {
    let time = player.current_position();
    let duration = player.duration();
    let playing = player.is_playing();

    println!(
        "Player at {:?} / {:?}, currently playing: {}",
        time, duration, playing
    );
}
