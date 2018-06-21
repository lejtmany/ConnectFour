struct Board {
    board:[[Square;6];7]
}

#[derive(Copy, Clone)]
struct Square {
    team: Option<Team>
}

#[derive(Copy, Clone)]
enum Team{
    TEAM1, TEAM2
}

impl Board {
    fn new()->Board{
        Board{board:[[Square{team:None};6];7]}
    }
}