"""
Game view for the Zappy GUI client - renders the game map
"""
from PyQt6.QtWidgets import QWidget, QGraphicsView, QGraphicsScene, QVBoxLayout, QLabel
from PyQt6.QtGui import (QColor, QBrush, QPen, QFont, QPolygonF, QPainter, 
                        QRadialGradient, QLinearGradient)
from PyQt6.QtCore import Qt, QPointF, QRectF, QTimer, pyqtSignal
import math
import random
import time


class GameView(QWidget):
    """Game view widget for rendering the Zappy game map"""
    
    def __init__(self):
        super().__init__()
        self.initUI()
        
        # Map properties
        self.map_width = 10  # default
        self.map_height = 10  # default
        self.cell_size = 60  # pixels
        
        # Colors for teams
        self.team_colors = {}
        self.color_index = 0
        self.predefined_colors = [
            QColor(255, 0, 0),    # Red
            QColor(0, 0, 255),    # Blue
            QColor(0, 255, 0),    # Green
            QColor(255, 165, 0),  # Orange
            QColor(128, 0, 128),  # Purple
            QColor(255, 192, 203),# Pink
            QColor(0, 255, 255),  # Cyan
            QColor(255, 255, 0),  # Yellow
        ]
        
        # Resource colors
        self.resource_colors = {
            0: QColor(240, 240, 160),  # Food (yellow)
            1: QColor(180, 180, 180),  # Linemate (gray)
            2: QColor(139, 69, 19),    # Deraumere (brown)
            3: QColor(0, 128, 128),    # Sibur (teal)
            4: QColor(255, 105, 180),  # Mendiane (pink)
            5: QColor(50, 205, 50),    # Phiras (green)
            6: QColor(138, 43, 226)    # Thystame (purple)
        }

        # Components
        self.scene = None
        self.view = None


    def clear(self):
        self.players = {}
        self.eggs = {}
        self.incantations = []
        self.tiles = {}
        self.broadcasts = []

    def initUI(self):
        """Initialize the UI components"""
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        
        # Create graphics view and scene
        self.scene = QGraphicsScene(self)
        self.view = QGraphicsView(self.scene)
        self.view.setRenderHint(QPainter.RenderHint.Antialiasing)
        self.view.setBackgroundBrush(QColor(50, 50, 50))  # Dark background
        
        layout.addWidget(self.view)
    
    def set_map_size(self, width, height):
        """Set or update the map size"""
        self.map_width = width
        self.map_height = height
        
        # Adjust cell size based on map dimensions
        view_width = self.view.viewport().width()
        view_height = self.view.viewport().height()
        
        self.cell_size = min(view_width / width, view_height / height) * 0.9
        
        # Update info
        self.info_label.setText(f"Map size: {width}x{height}")
    
    def draw_map(self):
        """Clear and redraw the entire map"""
        self.scene.clear()
        
        # Calculate map dimensions
        map_width_px = self.map_width * self.cell_size
        map_height_px = self.map_height * self.cell_size
        
        # Draw grid
        grid_pen = QPen(QColor(70, 70, 70))
        for i in range(self.map_width + 1):
            x = i * self.cell_size
            self.scene.addLine(x, 0, x, map_height_px, grid_pen)
        
        for i in range(self.map_height + 1):
            y = i * self.cell_size
            self.scene.addLine(0, y, map_width_px, y, grid_pen)

        # Draw tile contents
        for pos, resources in self.tiles.items():
            x, y = pos
            self.draw_tile(x, y, resources)
        
        # Draw eggs
        for egg_id, egg_data in self.eggs.items():
            self.draw_egg(egg_data)
        
        # Draw incantations
        for incantation in self.incantations:
            self.draw_incantation(incantation)
        
        # Draw players
        for player_id, player_data in self.players.items():
            self.draw_player(player_data)
        
        # Set scene rect and adjust view
        self.scene.setSceneRect(0, 0, map_width_px, map_height_px)
        self.view.fitInView(self.scene.sceneRect(), Qt.AspectRatioMode.KeepAspectRatio)
    
    def get_team_color(self, team_name):
        """Get a color for a team, creating one if needed"""
        if team_name not in self.team_colors:
            color = self.predefined_colors[self.color_index % len(self.predefined_colors)]
            self.team_colors[team_name] = color
            self.color_index += 1
        return self.team_colors[team_name]
    
    def draw_tile(self, x, y, resources):
        """Draw a tile with resources"""
        # Convert to pixel coordinates
        px = x * self.cell_size
        py = (self.map_height - y - 1) * self.cell_size  # Flip Y for visual consistency
        
        # Draw background
        tile_rect = QRectF(px, py, self.cell_size, self.cell_size)
        self.scene.addRect(tile_rect, QPen(Qt.GlobalColor.transparent), 
                           QBrush(QColor(30, 30, 30)))
        
        # Coordinates text
        coord_text = self.scene.addText(f"{x},{y}")
        coord_text.setDefaultTextColor(QColor(180, 180, 180))
        coord_text.setPos(px + 5, py + 5)
        coord_text.setFont(QFont("Arial", 8))
        
        # Draw resources
        if resources:
            resource_size = self.cell_size / 5
            spacing = resource_size * 1.2
            
            # Calculate how many resources to display (up to a reasonable limit)
            total_resources = sum(resources)
            max_display = 8  # Maximum resources to display per tile
            
            if total_resources > 0:
                # Position resources in a grid pattern
                grid_size = math.ceil(math.sqrt(max_display))
                
                displayed = 0
                for res_type, count in enumerate(resources):
                    for _ in range(min(count, max_display - displayed)):
                        if displayed >= max_display:
                            break
                            
                        # Calculate position in grid
                        grid_x = displayed % grid_size
                        grid_y = displayed // grid_size
                        
                        res_x = px + (grid_x + 1) * spacing
                        res_y = py + (grid_y + 1) * spacing
                        
                        # Draw resource
                        resource_rect = QRectF(res_x, res_y, resource_size, resource_size)
                        self.scene.addEllipse(resource_rect, 
                                             QPen(Qt.GlobalColor.transparent),
                                             QBrush(self.resource_colors.get(res_type, QColor(200, 200, 200))))
                        
                        displayed += 1
                    
                # If there are more resources than we can show, indicate with text
                if total_resources > max_display:
                    more_text = self.scene.addText(f"+{total_resources - max_display}")
                    more_text.setDefaultTextColor(QColor(255, 255, 255))
                    more_text.setPos(px + self.cell_size - 20, py + self.cell_size - 20)
    
    def draw_player(self, player_data):
        """Draw a player on the map"""
        player_id = player_data['id']
        x, y = player_data['x'], player_data['y']
        orientation = player_data['orientation']
        level = player_data.get('level', 1)
        team = player_data.get('team', 'unknown')
        
        # Convert to pixel coordinates
        px = (x + 0.5) * self.cell_size  # Center of cell
        py = (self.map_height - y - 0.5) * self.cell_size  # Flip Y, center of cell
        
        player_size = self.cell_size * 0.6
        
        # Base player shape (circle)
        player_rect = QRectF(px - player_size/2, py - player_size/2, player_size, player_size)
        
        # Create a radial gradient for the player
        team_color = self.get_team_color(team)
        gradient = QRadialGradient(px, py, player_size/2)
        gradient.setColorAt(0, team_color.lighter(150))
        gradient.setColorAt(1, team_color)
        
        player_circle = self.scene.addEllipse(player_rect, 
                                             QPen(Qt.GlobalColor.black, 1),
                                             QBrush(gradient))
        player_circle.setZValue(10)  # Draw players above other elements
        
        # Direction indicator (triangle)
        direction_size = player_size * 0.4
        
        # Calculate the points for the direction triangle based on orientation
        angle_rad = {1: math.pi/2, 2: 0, 3: -math.pi/2, 4: math.pi}[orientation]
        
        dx = math.cos(angle_rad) * direction_size
        dy = -math.sin(angle_rad) * direction_size  # Negative because Y is flipped in Qt
        
        triangle = QPolygonF()
        triangle.append(QPointF(px + dx, py + dy))
        triangle.append(QPointF(px + dy*0.5, py - dx*0.5))
        triangle.append(QPointF(px - dy*0.5, py + dx*0.5))
        
        direction_indicator = self.scene.addPolygon(triangle, 
                                                  QPen(Qt.GlobalColor.black, 1),
                                                  QBrush(QColor(255, 255, 255, 200)))
        direction_indicator.setZValue(11)
        
        # Player ID and level text
        id_text = self.scene.addText(f"#{player_id}")
        id_text.setDefaultTextColor(QColor(255, 255, 255))
        id_text.setPos(px - id_text.boundingRect().width()/2, 
                       py - player_size/2 - id_text.boundingRect().height())
        id_text.setZValue(12)
        
        level_text = self.scene.addSimpleText(f"L{level}")
        level_text.setBrush(QColor(255, 255, 200))
        level_text.setPos(px - level_text.boundingRect().width()/2, 
                          py + player_size/2)
        level_text.setZValue(12)