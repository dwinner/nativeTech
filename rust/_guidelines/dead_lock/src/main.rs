use std::collections::HashMap;
use std::sync::Mutex;

fn main()
{
   println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct Player
{
   name: String,
}

struct Game {}

impl Game
{
   fn add_player(&self, username: &str) -> bool
   {
      todo!()
   }
   
   fn has_player(&self, username: &str) -> bool
   {
      todo!()
   }

   fn remove_player(&self, username: &str)
   {
      todo!()
   }
}

#[derive(Debug, Copy, Clone)]
struct GameId
{
   id: u32,
}

struct GameServer
{
   // Map for player name -> player info
   players: Mutex<HashMap<String, Player>>,
   // Current games, indexed by unique ID
   games: Mutex<HashMap<GameId, Game>>,
   // had better use something like:
   // state: Mutex<GameState>
   // for `consistency`
}

impl GameServer
{
   /// Adding player for joining to the current game
   fn add_and_join(&self, username: &str, info: Player) -> Option<GameId>
   {
      // adding player
      {
         let mut players = self.players.lock().unwrap();
         players.insert(username.to_owned(), info);
      }

      // Search a game with available place for a new player
      {
         let mut games = self.games.lock().unwrap();
         for (id, game) in games.iter_mut()
         {
            if game.add_player(username)
            {
               return Some(id.clone());
            }
         }
      }

      None
   }

   /// Ban a player `username` and removing from the current games
   fn ban_player(&self, username: &str)
   {
      // Search all the games where a player exists and removing
      {
         let mut games = self.games.lock().unwrap();
         games
            .iter_mut()
            .filter(|(_id, g)| g.has_player(username))
            .for_each(|(_id, g)| g.remove_player(username));
      }

      // remove a user
      {
         let mut players = self.players.lock().unwrap();
         players.remove(username);
      }
   }
}
