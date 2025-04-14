"""
Team panel component for the Zappy GUI client
"""
from PyQt6.QtWidgets import (QWidget, QVBoxLayout, QHBoxLayout, QLabel, 
                            QListWidget, QListWidgetItem, QTableWidget, 
                            QTableWidgetItem, QHeaderView, QGroupBox)
from PyQt6.QtCore import Qt, pyqtSignal
from PyQt6.QtGui import QColor, QBrush


class TeamPanel(QWidget):
    """Panel for displaying team information"""
    
    # Signals
    team_selected = pyqtSignal(str)  # Team name
    
    def __init__(self):
        super().__init__()
        
        self.teams = {}  # team_name -> team data
        
        self.init_ui()
    
    def init_ui(self):
        """Initialize the UI components"""
        layout = QVBoxLayout(self)
        
        # Team list
        teams_group = QGroupBox("Teams")
        teams_layout = QVBoxLayout(teams_group)
        
        self.teams_list = QListWidget()
        self.teams_list.currentItemChanged.connect(self.on_team_selected)
        teams_layout.addWidget(self.teams_list)
        
        layout.addWidget(teams_group)
        
        # Team details
        details_group = QGroupBox("Team Details")
        details_layout = QVBoxLayout(details_group)
        
        self.details_label = QLabel("No team selected")
        details_layout.addWidget(self.details_label)
        
        # Player table
        self.players_table = QTableWidget(0, 3)  # Columns: Player ID, Level, Status
        self.players_table.setHorizontalHeaderLabels(["Player ID", "Level", "Status"])
        self.players_table.horizontalHeader().setSectionResizeMode(QHeaderView.ResizeMode.Stretch)
        details_layout.addWidget(self.players_table)
        
        # Eggs
        self.egg_label = QLabel("Eggs: 0")
        details_layout.addWidget(self.egg_label)
        
        layout.addWidget(details_group)
        
        # Fill remaining space
        layout.addStretch(1)
    
    def add_team(self, team_name):
        """Add a new team to the panel"""
        if team_name in self.teams:
            return
            
        self.teams[team_name] = {
            'name': team_name,
            'players': {},
            'eggs': []
        }
        
        # Add to list widget
        self.teams_list.addItem(team_name)
    
    def add_player_to_team(self, team_name, player_id):
        """Add a player to a team"""
        if team_name not in self.teams:
            self.add_team(team_name)
            
        if player_id in self.teams[team_name]['players']:
            return
            
        self.teams[team_name]['players'][player_id] = {
            'id': player_id,
            'level': 1,
            'status': 'Alive'
        }
        
        # Update the display if this team is selected
        if self.get_selected_team() == team_name:
            self.update_team_display(team_name)
    
    def update_player_level(self, team_name, player_id, level):
        """Update a player's level in a team"""
        if team_name not in self.teams:
            return
            
        if player_id not in self.teams[team_name]['players']:
            return
            
        self.teams[team_name]['players'][player_id]['level'] = level
        
        # Update the display if this team is selected
        if self.get_selected_team() == team_name:
            self.update_team_display(team_name)
    
    def update_player_status(self, team_name, player_id, status):
        """Update a player's status in a team"""
        if team_name not in self.teams:
            return
            
        if player_id not in self.teams[team_name]['players']:
            return
            
        self.teams[team_name]['players'][player_id]['status'] = status
        
        # Update the display if this team is selected
        if self.get_selected_team() == team_name:
            self.update_team_display(team_name)
    
    def remove_player_from_team(self, team_name, player_id):
        """Remove a player from a team"""
        if team_name not in self.teams:
            return
            
        if player_id not in self.teams[team_name]['players']:
            return
            
        del self.teams[team_name]['players'][player_id]
        
        # Update the display if this team is selected
        if self.get_selected_team() == team_name:
            self.update_team_display(team_name)
    
    def add_egg_to_team(self, team_name, egg_id):
        """Add an egg to a team"""
        if team_name not in self.teams:
            self.add_team(team_name)
            
        self.teams[team_name]['eggs'].append(egg_id)
        
        # Update the display if this team is selected
        if self.get_selected_team() == team_name:
            self.update_team_display(team_name)
    
    def remove_egg_from_team(self, team_name, egg_id):
        """Remove an egg from a team"""
        if team_name not in self.teams:
            return
            
        if egg_id not in self.teams[team_name]['eggs']:
            return
            
        self.teams[team_name]['eggs'].remove(egg_id)
        
        # Update the display if this team is selected
        if self.get_selected_team() == team_name:
            self.update_team_display(team_name)
    
    def get_selected_team(self):
        """Get the currently selected team"""
        current_item = self.teams_list.currentItem()
        if current_item is None:
            return None
            
        return current_item.text()
    
    def update_team_display(self, team_name):
        """Update the display for a specific team"""
        if team_name not in self.teams:
            return
            
        team = self.teams[team_name]
        
        # Update details label
        player_count = len(team['players'])
        egg_count = len(team['eggs'])
        self.details_label.setText(f"Team: {team_name}\nPlayers: {player_count}")
        self.egg_label.setText(f"Eggs: {egg_count}")
        
        # Update player table
        self.players_table.setRowCount(player_count)
        
        for row, (player_id, player) in enumerate(team['players'].items()):
            # Player ID
            id_item = QTableWidgetItem(f"#{player_id}")
            id_item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
            self.players_table.setItem(row, 0, id_item)
            
            # Level
            level_item = QTableWidgetItem(str(player['level']))
            level_item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
            self.players_table.setItem(row, 1, level_item)
            
            # Status with color
            status_item = QTableWidgetItem(player['status'])
            status_item.setTextAlignment(Qt.AlignmentFlag.AlignCenter)
            
            # Set color based on status
            if player['status'] == 'Alive':
                status_item.setForeground(QBrush(QColor(0, 128, 0)))  # Green
            elif player['status'] == 'Dead':
                status_item.setForeground(QBrush(QColor(255, 0, 0)))  # Red
            
            self.players_table.setItem(row, 2, status_item)
    
    def on_team_selected(self, current, previous):
        """Handle team selection in list widget"""
        if current is None:
            self.details_label.setText("No team selected")
            self.players_table.setRowCount(0)
            return
            
        team_name = current.text()
        self.update_team_display(team_name)
        self.team_selected.emit(team_name)