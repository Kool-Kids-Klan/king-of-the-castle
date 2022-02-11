extern crate kotc_database;

use kotc_database::repo::{
    game_repo::GameRepo, participation_repo::ParticipationRepo, user_repo::UserRepo,
};
use kotc_database::{get_game_repo, get_participation_repo, get_user_repo};

use kotc_game::game::ws_messages::ServerMessage;
use kotc_game::game::Game;

#[actix_rt::test]
async fn db_test() -> anyhow::Result<()> {
    let user_repo = get_user_repo().await;
    let game_repo = get_game_repo().await;
    let participation_repo = get_participation_repo().await;

    let user_id = user_repo
        .create_user("Puckoland", "k@gmail.com", "asdfjkn")
        .await?;
    let user = user_repo.get_user(user_id).await?;
    println!("{:?}", user);
    let user2_id = user_repo.create_user("Dante", "d@gmail.com", "bab").await?;
    let user2 = user_repo.get_user(user2_id).await?;
    println!("{:?}", user2);

    let game_id = game_repo.create_game().await?;
    let game = game_repo.get_game(game_id).await?;
    println!("{:?}", game);

    let participation_id = participation_repo
        .create_participation(game_id, user_id)
        .await?;
    let participation = participation_repo
        .get_participation(participation_id)
        .await?;
    println!("{:?}", participation);

    let won_game = game_repo.update_game_winner(game_id, user_id).await?;
    println!("{:?}", won_game);
    let user = user_repo
        .update_user(user_id, Some("new name"), None, Some("password"))
        .await?;
    println!("{:?}", user);

    participation_repo
        .delete_participation(participation_id)
        .await?;
    game_repo.delete_game(game_id).await?;
    // user_repo.delete_user(user_id).await?;

    let mut game = Game::new();

    println!("{:?}", game.connect_player(user_id).await);
    println!("{:?}", game.connect_player(99999).await);
    println!("{:?}", game.disconnect_player(user_id));
    println!("{:?}", game.connect_player(user_id).await);
    println!("{:?}", game.connect_player(user_id).await);
    println!("{:?}", game.connect_player(user2_id).await);

    println!("{:?}", game.player_flip_ready(user_id).await);
    println!("{:?}", game.player_flip_ready(user2_id).await);

    println!("{:?}", game.make_action(99999, 0, 1).await);
    println!("{:?}", game.make_action(user2_id, 0, 1).await);
    println!("{:?}", game.make_action(user_id, 0, 10).await);
    println!("{:?}", game.make_action(user_id, 20, 0).await);

    for _ in 0..6 {
        print_messages(game.make_action(user_id, 0, 0).await);
        print_messages(game.make_action(user2_id, 0, 0).await);
        print_messages(game.make_action(user_id, 0, 0).await);
        print_messages(game.make_action(user2_id, 0, 0).await);
        print_messages(game.make_action(user_id, 0, 0).await);
        print_messages(game.make_action(user2_id, 0, 0).await);
        print_messages(game.make_action(user_id, 0, 0).await);
        print_messages(game.make_action(user2_id, 0, 0).await);

        print_messages(game.make_action(user_id, 1, 0).await);
        print_messages(game.make_action(user2_id, 1, 0).await);
        print_messages(game.make_action(user_id, 1, 0).await);
        print_messages(game.make_action(user2_id, 1, 0).await);
        print_messages(game.make_action(user_id, 1, 0).await);
        print_messages(game.make_action(user2_id, 1, 0).await);
        print_messages(game.make_action(user_id, 1, 0).await);
        print_messages(game.make_action(user2_id, 1, 0).await);
    }

    Ok(())
}

fn print_messages(messages: Vec<ServerMessage>) {
    println!("---");
    messages
        .iter()
        .for_each(|message| println!("{:?}", message))
}
