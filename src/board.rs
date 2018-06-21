struct Board {
    board:[[Square;7];6]
}

struct Square {
    square: Option<Team>
}

enum Team{
    TEAM1, TEAM2
}