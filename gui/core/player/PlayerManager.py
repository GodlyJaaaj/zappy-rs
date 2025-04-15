from PyQt6.QtCore import QObject, pyqtSignal

DIRECTIONS = {"N", "E", "S", "W"}
RESOURCES = {"food", "linemate", "deraumere", "sibur", "mendiane", "phiras", "thystame"}

class PlayerManager(QObject):
    player_added = pyqtSignal(int)        # player_id
    player_updated = pyqtSignal(int)      # player_id
    player_removed = pyqtSignal(int)      # player_id

    def __init__(self):
        super().__init__()
        self.players = {}  # player_id -> player dict

    def add_player(self, player_id: int, position: tuple[int, int], team: str, direction: str, level: int):
        """Ajoute un nouveau joueur avec ses attributs"""
        if player_id <= 0:
            raise ValueError("L'ID du joueur doit être un entier > 0")
        if not (isinstance(position, tuple) and len(position) == 2 and all(isinstance(v, int) and v >= 0 for v in position)):
            raise ValueError("La position doit être un tuple (x, y) d'entiers positifs")
        if direction not in DIRECTIONS:
            raise ValueError("Direction invalide")
        if level < 1:
            raise ValueError("Le niveau d'incantation doit être >= 1")

        self.players[player_id] = {
            "position": position,
            "team": team,
            "direction": direction,
            "level": level
        }
        self.player_added.emit(player_id)

    def update_player(self, player_id: int, field: str, value):
        """Met à jour un champ arbitraire d’un joueur"""
        if player_id in self.players and field in self.players[player_id]:
            self.players[player_id][field] = value
            self.player_updated.emit(player_id)

    def update_position(self, player_id: int, new_position: tuple[int, int]):
        if player_id in self.players:
            self.players[player_id]["position"] = new_position
            self.player_updated.emit(player_id)

    def update_direction(self, player_id: int, new_direction: str):
        if player_id in self.players and new_direction in DIRECTIONS:
            self.players[player_id]["direction"] = new_direction
            self.player_updated.emit(player_id)

    def remove_player(self, player_id: int):
        if player_id in self.players:
            del self.players[player_id]
            self.player_removed.emit(player_id)

    def get(self, player_id: int):
        return self.players.get(player_id)

    def all_players(self):
        return self.players

    def players_by_team(self, team_name: str) -> dict[int, dict]:
        return {pid: p for pid, p in self.players.items() if p["team"] == team_name}

    def update_player_position(self, player_id: int, position: tuple[int, int], direction: str):
        """Update both position and direction at once"""
        if player_id in self.players:
            self.players[player_id]["position"] = position
            if direction in DIRECTIONS:
                self.players[player_id]["direction"] = direction
            self.player_updated.emit(player_id)
